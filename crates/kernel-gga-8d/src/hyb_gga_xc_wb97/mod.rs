//! HYB_GGA_XC_WB97 kernel -- incremental derivative structure.

//! unpol: preamble=179 lines
//!   exc: shared=0, delta=179, outputs=1
//!   vxc: shared=179, delta=164, outputs=3
//!   fxc: shared=343, delta=231, outputs=6
//!   kxc: shared=574, delta=274, outputs=10
//!   lxc: shared=848, delta=186, outputs=15
//! pol: preamble=328 lines
//!   exc: shared=0, delta=328, outputs=1
//!   vxc: shared=328, delta=485, outputs=6
//!   fxc: shared=813, delta=1140, outputs=21
//!   kxc: shared=1953, delta=2399, outputs=56
//!   lxc: shared=4352, delta=2840, outputs=126

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
