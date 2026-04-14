//! GGA_X_OPTX kernel -- incremental derivative structure.

//! unpol: preamble=31 lines
//!   exc: shared=0, delta=31, outputs=1
//!   vxc: shared=31, delta=17, outputs=3
//!   fxc: shared=48, delta=30, outputs=6
//!   kxc: shared=78, delta=31, outputs=10
//!   lxc: shared=109, delta=13, outputs=15
//! pol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=54, outputs=6
//!   fxc: shared=109, delta=110, outputs=21
//!   kxc: shared=219, delta=184, outputs=56
//!   lxc: shared=403, delta=232, outputs=126

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
