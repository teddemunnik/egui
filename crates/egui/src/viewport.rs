use std::{fmt::Display, sync::Arc};

use crate::Context;

/// This is used to send a command to a specific viewport
///
/// This is returned by `Context::get_viewport_id` and `Context::get_parent_viewport_id`
#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ViewportId(pub(crate) u64);

impl Display for ViewportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ViewportId {
    /// This will return the `ViewportId` of the main viewport
    pub const MAIN: Self = Self(0);
}

/// This is used to render an async viewport
pub type ViewportRender = dyn Fn(&Context) + Sync + Send;

/// The filds in this struct should not be change directly, but is not problem tho!
/// Every thing is wraped in Option<> indicates that thing should not be changed!
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct ViewportBuilder {
    pub title: String,
    pub name: Option<String>,
    pub position: Option<Option<(i32, i32)>>,
    pub inner_size: Option<Option<(u32, u32)>>,
    pub fullscreen: Option<bool>,
    pub maximized: Option<bool>,
    pub resizable: Option<bool>,
    pub transparent: Option<bool>,
    pub decorations: Option<bool>,
    pub icon: Option<Option<Arc<(u32, u32, Vec<u8>)>>>,
    pub active: Option<bool>,
    pub visible: Option<bool>,
    pub title_hidden: Option<bool>,
    pub titlebar_transparent: Option<bool>,
    pub fullsize_content_view: Option<bool>,
    pub min_inner_size: Option<Option<(u32, u32)>>,
    pub max_inner_size: Option<Option<(u32, u32)>>,
    pub drag_and_drop: Option<bool>,

    pub close_button: Option<bool>,
}

impl Default for ViewportBuilder {
    fn default() -> Self {
        Self {
            title: "Dummy EGUI Window".into(),
            name: None,
            position: None,
            inner_size: Some(Some((300, 200))),
            fullscreen: None,
            maximized: None,
            resizable: Some(true),
            transparent: Some(true),
            decorations: Some(true),
            icon: None,
            active: Some(true),
            visible: Some(true),
            title_hidden: None,
            titlebar_transparent: None,
            fullsize_content_view: None,
            min_inner_size: None,
            max_inner_size: None,
            drag_and_drop: None,
            close_button: None,
        }
    }
}

impl ViewportBuilder {
    pub fn empty() -> Self {
        Self {
            title: "Dummy EGUI Window".into(),
            name: None,
            position: None,
            inner_size: None,
            fullscreen: None,
            maximized: None,
            resizable: None,
            transparent: None,
            decorations: None,
            icon: None,
            active: None,
            visible: None,
            title_hidden: None,
            titlebar_transparent: None,
            fullsize_content_view: None,
            min_inner_size: None,
            max_inner_size: None,
            drag_and_drop: None,
            close_button: None,
        }
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = Some(decorations);
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = Some(fullscreen);
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = Some(maximized);
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = Some(transparent);
        self
    }

    /// The icon needs to be wraped in Arc because will be copyed every frame
    pub fn with_window_icon(mut self, icon: Option<Arc<(u32, u32, Vec<u8>)>>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn with_title_hidden(mut self, title_hidden: bool) -> Self {
        self.title_hidden = Some(title_hidden);
        self
    }

    pub fn with_titlebar_transparent(mut self, value: bool) -> Self {
        self.titlebar_transparent = Some(value);
        self
    }

    pub fn with_fullsize_content_view(mut self, value: bool) -> Self {
        self.fullsize_content_view = Some(value);
        self
    }

    pub fn with_inner_size(mut self, value: Option<(u32, u32)>) -> Self {
        self.inner_size = Some(value);
        self
    }

    pub fn with_min_inner_size(mut self, value: Option<(u32, u32)>) -> Self {
        self.min_inner_size = Some(value);
        self
    }

    pub fn with_max_inner_size(mut self, value: Option<(u32, u32)>) -> Self {
        self.max_inner_size = Some(value);
        self
    }

    pub fn with_close_button(mut self, value: bool) -> Self {
        self.close_button = Some(value);
        self
    }

    pub fn with_drag_and_drop(mut self, value: bool) -> Self {
        self.drag_and_drop = Some(value);
        self
    }

    pub fn with_position(mut self, value: Option<(i32, i32)>) -> Self {
        self.position = Some(value);
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ViewportCommand {
    Drag,
    InnerSize(u32, u32),
    /// Top, Bottom, Right, Left
    Resize(bool, bool, bool, bool),
}
