//! GGA_XC_TH3 kernel -- incremental derivative structure.

//! unpol: preamble=54 lines
//!   exc: shared=0, delta=54, outputs=1
//!   vxc: shared=54, delta=20, outputs=3
//!   fxc: shared=74, delta=18, outputs=6
//!   kxc: shared=92, delta=21, outputs=10
//!   lxc: shared=113, delta=13, outputs=15
//! pol: preamble=122 lines
//!   exc: shared=0, delta=122, outputs=1
//!   vxc: shared=122, delta=109, outputs=6
//!   fxc: shared=231, delta=258, outputs=21
//!   kxc: shared=489, delta=408, outputs=56
//!   lxc: shared=897, delta=485, outputs=126

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
