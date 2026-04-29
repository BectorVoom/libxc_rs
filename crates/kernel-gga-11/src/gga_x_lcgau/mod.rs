//! GGA_X_LCGAU kernel -- incremental derivative structure.

//! unpol: preamble=124 lines
//!   exc: shared=0, delta=124, outputs=1
//!   vxc: shared=124, delta=144, outputs=3
//!   fxc: shared=268, delta=308, outputs=6
//!   kxc: shared=576, delta=447, outputs=10
//!   lxc: shared=1023, delta=443, outputs=15
//! pol: preamble=241 lines
//!   exc: shared=0, delta=241, outputs=1
//!   vxc: shared=241, delta=389, outputs=6
//!   fxc: shared=630, delta=1137, outputs=21
//!   kxc: shared=1767, delta=2045, outputs=56
//!   lxc: shared=3812, delta=3254, outputs=126

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
