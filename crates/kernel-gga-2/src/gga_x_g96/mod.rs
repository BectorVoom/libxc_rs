//! GGA_X_G96 kernel -- incremental derivative structure.

//! unpol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=13, outputs=3
//!   fxc: shared=39, delta=25, outputs=6
//!   kxc: shared=64, delta=29, outputs=10
//!   lxc: shared=93, delta=16, outputs=15
//! pol: preamble=49 lines
//!   exc: shared=0, delta=49, outputs=1
//!   vxc: shared=49, delta=44, outputs=6
//!   fxc: shared=93, delta=114, outputs=21
//!   kxc: shared=207, delta=210, outputs=56
//!   lxc: shared=417, delta=304, outputs=126

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
