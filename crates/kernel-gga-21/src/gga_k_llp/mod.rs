//! GGA_K_LLP kernel -- incremental derivative structure.

//! unpol: preamble=40 lines
//!   exc: shared=0, delta=40, outputs=1
//!   vxc: shared=40, delta=28, outputs=3
//!   fxc: shared=68, delta=45, outputs=6
//!   kxc: shared=113, delta=69, outputs=10
//!   lxc: shared=182, delta=31, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=70, outputs=6
//!   fxc: shared=136, delta=136, outputs=21
//!   kxc: shared=272, delta=251, outputs=56
//!   lxc: shared=523, delta=263, outputs=126

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
