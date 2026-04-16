//! GGA_X_AK13 kernel -- incremental derivative structure.

//! unpol: preamble=34 lines
//!   exc: shared=0, delta=34, outputs=1
//!   vxc: shared=34, delta=33, outputs=3
//!   fxc: shared=67, delta=49, outputs=6
//!   kxc: shared=116, delta=66, outputs=10
//!   lxc: shared=182, delta=47, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=68, outputs=6
//!   fxc: shared=125, delta=149, outputs=21
//!   kxc: shared=274, delta=253, outputs=56
//!   lxc: shared=527, delta=310, outputs=126

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
