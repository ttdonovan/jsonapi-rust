
extern crate serde_json;

use std::collections::HashMap;

pub type JsonApiValue = serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Resource {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub attributes: ResourceAttributes,
    pub relationships: Option<Relationships>,
    pub links: Option<Links>,
}

pub type Resources = Vec<Resource>;
pub type Links = HashMap<String, String>;
pub type Meta = HashMap<String, String>;
pub type ResourceAttributes = HashMap<String, JsonApiValue>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Relationship {
    pub data: ResourceIdentifier,
}
pub type Relationships = HashMap<String, Relationship>;

pub type Included = Vec<Resource>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PrimaryData {
    None,
    Single(Resource),
    Multiple(Resources),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonApiResponse {
    pub data: PrimaryData,
    pub included: Option<Resources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonApiError {
    pub id: String,
    pub links: Links,
    pub status: String,
    pub code: String,
    pub title: String,
    pub detail: String,
    pub source: Option<ErrorSource>,
    pub meta: Option<Meta>,
}

pub type JsonApiErrors = Vec<JsonApiError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

impl Pagination {
    pub fn has_first(&self) -> bool {
        !self.first.is_none()
    }
    pub fn has_prev(&self) -> bool {
        !self.prev.is_none()
    }
    pub fn has_next(&self) -> bool {
        !self.next.is_none()
    }
    pub fn has_last(&self) -> bool {
        !self.last.is_none()
    }
}

// Spec says at least one of data, errors, meta
// data and errors must not co-exist
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub data: Option<PrimaryData>,
    pub errors: Option<JsonApiErrors>,
    pub meta: Option<Meta>,
}

impl Document {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_none()
    }
    pub fn has_data(&self) -> bool {
        !self.data.is_none()
    }
    pub fn has_meta(&self) -> bool {
        !self.meta.is_none()
    }
    pub fn is_valid(&self) -> bool {
        self.has_data() ^ self.has_errors()
    }
}