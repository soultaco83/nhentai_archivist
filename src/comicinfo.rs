// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
#![allow(non_snake_case)] // non snake case because XML does this convention
use crate::api_response::*;
use crate::hentai::*;


/// # Summary
/// ComicInfo.xml schema for tags: https://anansi-project.github.io/docs/comicinfo/documentation
/// Komga interpretation: https://komga.org/docs/guides/scan-analysis-refresh/#import-metadata-for-cbrcbz-containing-a-comicinfoxml-file
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct ComicInfo // ComicInfo.xml schema, must be named "ComicInfo" and not "ComicInfoXml" because standard defines root tag as "ComicInfo" and serde serialisation uses struct name as root tag
{
    pub Title: String, // pretty title
    pub Year: i16, // upload year
    pub Month: u8, // upload month
    pub Day: u8, // upload day
    pub Writer: Option<String>, // tag type: artist
    pub Translator: Option<String>, // scanlator
    pub Publisher: Option<String>, // tag type: group
    pub Genre: Option<String>, // tag type: category
    pub Tags: Option<String>, // tag types: character, language, parody, tag; language does not get own field "LanguageISO" because it only interprets 1 language as code properly, exhaustive language code list and only keeping 1 language if multiple present is janky
    pub Web: String, // nhentai gallery

}


impl From<Hentai> for ComicInfo
{
    fn from(hentai: Hentai) -> Self // Hentai -> ComicInfo
    {
        return Self
        {
            Title: format!("{} {}", hentai.id, hentai.title_pretty.unwrap_or_default()), // id and actual title, because can't search for field "Number" in komga
            Year: hentai.upload_date.format("%Y").to_string().parse::<i16>().unwrap_or_else(|_| panic!("Converting year \"{}\" to i16 failed even though it comes directly from chrono::DateTime.", hentai.upload_date.format("%Y"))),
            Month: hentai.upload_date.format("%m").to_string().parse::<u8>().unwrap_or_else(|_| panic!("Converting month \"{}\" to u8 failed even though it comes directly from chrono::DateTime.", hentai.upload_date.format("%m"))),
            Day: hentai.upload_date.format("%d").to_string().parse::<u8>().unwrap_or_else(|_| panic!("Converting day \"{}\" to u8 failed even though it comes directly from chrono::DateTime.", hentai.upload_date.format("%d"))),
            Writer: filter_and_combine_tags(&hentai.tags, &["artist"], false),
            Translator: hentai.scanlator,
            Publisher: filter_and_combine_tags(&hentai.tags, &["group"], false),
            Genre: filter_and_combine_tags(&hentai.tags, &["category"], false),
            Web: format!("https://nhentai.net/g/{id}/", id=hentai.id),
            Tags: filter_and_combine_tags(&hentai.tags, &["character", "language", "parody", "tag"], true),
        }
    }
}


/// # Summary
/// Filters tags by type and combines the remaining into a single string. If no tags are found, returns None.
///
/// # Arguments
/// - `tags`: tag list to combine
/// - `types`: tag types to keep
/// - `display_type`: whether to display the tag type in the output in form of "type: name"
///
/// # Returns
/// - filtered and combined tags or None
fn filter_and_combine_tags(tags: &[Tag], types: &[&str], display_type: bool) -> Option<String>
{
    let mut tags_filtered: Vec<String> = tags.iter()
        .filter(|tag| types.contains(&tag.r#type.as_str())) // only keep tags with type in types
        .map
        (
            |tag|
            {
                if display_type {format!("{}: {}", tag.r#type, tag.name)}
                else {tag.name.clone()}
            }
        ) // change either to "{name}" or "{type}: {name}", because ComicInfo.xml + Komga don't have proper fields for all tag types
        .collect();
    tags_filtered.sort(); // sort alphabetically
    let tags_filtered_combined: Option<String> = Some(tags_filtered.join(",")) // join at ","
        .and_then(|s| if s.is_empty() {None} else {Some(s)}); // convert Some("") to None, otherwise forward unchanged

    return tags_filtered_combined;
}