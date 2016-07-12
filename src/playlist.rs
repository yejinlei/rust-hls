
use std::str::FromStr;
use std::string::ToString;
use std::collections::BTreeMap;

// Play List (M3U8 Format)
pub enum Tag {
    // Basic Tags
    M3U,
    VERSION,
    // Media Segment Tags
    INF,
    BYTERANGE,
    DISCONTINUITY,
    KEY,
    MAP,
    PROGRAM_DATE_TIME,
    DATERANGE,
    // Media Playlist Tags
    TARGETDURATION,
    MEDIA_SEQUENCE,
    DISCONTINUITY_SEQUENCE,
    ENDLIST,
    PLAYLIST_TYPE,
    I_FRAMES_ONLY,
    // Master Playlist Tags
    MEDIA,
    STREAM_INF,
    I_FRAME_STREAM_INF,
    SESSION_DATA,
    SESSION_KEY,
    // Media or Master Playlist Tags
    INDEPENDENT_SEGMENTS,
    START,
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        match *self {
            Tag::M3U            => "EXTM3U",
            Tag::VERSION        => "EXT-X-VERSION",
            Tag::INF            => "EXTINF",
            Tag::BYTERANGE      => "EXT-X-BYTERANGE",
            Tag::DISCONTINUITY  => "EXT-X-DISCONTINUITY",
            Tag::KEY            => "EXT-X-KEY",
            Tag::MAP            => "EXT-X-MAP",
            Tag::PROGRAM_DATE_TIME  => "EXT-X-PROGRAM-DATE-TIME",
            Tag::DATERANGE          => "EXT-X-DATERANGE",
            Tag::TARGETDURATION     => "EXT-X-TARGETDURATION",
            Tag::MEDIA_SEQUENCE     => "EXT-X-MEDIA-SEQUENCE",
            Tag::DISCONTINUITY_SEQUENCE => "EXT-X-DISCONTINUITY-SEQUENCE",
            Tag::ENDLIST        => "EXT-X-ENDLIST",
            Tag::PLAYLIST_TYPE  => "EXT-X-PLAYLIST-TYPE",
            Tag::I_FRAMES_ONLY  => "EXT-X-I-FRAMES-ONLY",
            Tag::MEDIA          => "EXT-X-MEDIA",
            Tag::STREAM_INF     => "EXT-X-STREAM-INF",
            Tag::I_FRAME_STREAM_INF     => "EXT-X-I-FRAME-STREAM-INF",
            Tag::SESSION_DATA           => "EXT-X-SESSION-DATA",
            Tag::SESSION_KEY            => "EXT-X-SESSION-KEY",
            Tag::INDEPENDENT_SEGMENTS   => "EXT-X-INDEPENDENT-SEGMENTS",
            Tag::START                  => "EXT-X-START",
        }.to_string()
    }
}

impl FromStr for Tag {
    type Err = ();
    fn from_str(s: &str) -> Result<Tag, ()> {
        match s {
            "EXTM3U" => Ok(Tag::M3U),
            "EXT-X-VERSION" => Ok(Tag::VERSION),
            _ => Err(())
        }
    }
}


#[derive(Debug)]
pub struct Playlist {
    map: BTreeMap<>
}

impl Playlist {
    pub fn new (duration: usize) -> Playlist {

    }
    pub fn append(&self) -> bool {

    }
}