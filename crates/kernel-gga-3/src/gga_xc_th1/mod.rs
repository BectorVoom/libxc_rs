//! GGA_XC_TH1 kernel -- incremental derivative structure.

//! unpol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=15, outputs=3
//!   fxc: shared=67, delta=18, outputs=6
//!   kxc: shared=85, delta=19, outputs=10
//!   lxc: shared=104, delta=14, outputs=15
//! pol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=111, outputs=6
//!   fxc: shared=227, delta=265, outputs=21
//!   kxc: shared=492, delta=433, outputs=56
//!   lxc: shared=925, delta=508, outputs=126

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
