//! GGA_C_LYPR kernel -- incremental derivative structure.

//! unpol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=55, outputs=3
//!   fxc: shared=110, delta=86, outputs=6
//!   kxc: shared=196, delta=119, outputs=10
//!   lxc: shared=315, delta=57, outputs=15
//! pol: preamble=106 lines
//!   exc: shared=0, delta=106, outputs=1
//!   vxc: shared=106, delta=186, outputs=6
//!   fxc: shared=292, delta=466, outputs=21
//!   kxc: shared=758, delta=1108, outputs=56
//!   lxc: shared=1866, delta=1705, outputs=126

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
