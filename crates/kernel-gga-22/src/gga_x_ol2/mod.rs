//! GGA_X_OL2 kernel -- incremental derivative structure.

//! unpol: preamble=29 lines
//!   exc: shared=0, delta=29, outputs=1
//!   vxc: shared=29, delta=19, outputs=3
//!   fxc: shared=48, delta=30, outputs=6
//!   kxc: shared=78, delta=36, outputs=10
//!   lxc: shared=114, delta=15, outputs=15
//! pol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=55, outputs=6
//!   fxc: shared=111, delta=120, outputs=21
//!   kxc: shared=231, delta=199, outputs=56
//!   lxc: shared=430, delta=235, outputs=126

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
