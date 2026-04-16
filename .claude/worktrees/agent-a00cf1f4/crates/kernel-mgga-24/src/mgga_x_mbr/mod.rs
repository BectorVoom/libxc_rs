//! MGGA_X_MBR kernel -- incremental derivative structure.

//! unpol: preamble=68 lines
//!   exc: shared=0, delta=68, outputs=1
//!   vxc: shared=68, delta=85, outputs=5
//!   fxc: shared=153, delta=326, outputs=15
//!   kxc: shared=479, delta=1462, outputs=35
//!   lxc: shared=1941, delta=3091, outputs=70
//! pol: preamble=113 lines
//!   exc: shared=0, delta=113, outputs=1
//!   vxc: shared=113, delta=177, outputs=10
//!   fxc: shared=290, delta=759, outputs=55
//!   kxc: shared=1049, delta=3561, outputs=220
//!   lxc: shared=4610, delta=9675, outputs=715

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
