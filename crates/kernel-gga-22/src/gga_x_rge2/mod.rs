//! GGA_X_RGE2 kernel -- incremental derivative structure.

//! unpol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=16, outputs=3
//!   fxc: shared=51, delta=23, outputs=6
//!   kxc: shared=74, delta=36, outputs=10
//!   lxc: shared=110, delta=25, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=51, outputs=6
//!   fxc: shared=108, delta=110, outputs=21
//!   kxc: shared=218, delta=226, outputs=56
//!   lxc: shared=444, delta=329, outputs=126

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
