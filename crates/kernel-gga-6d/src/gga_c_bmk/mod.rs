//! GGA_C_BMK kernel -- incremental derivative structure.

//! unpol: preamble=121 lines
//!   exc: shared=0, delta=121, outputs=1
//!   vxc: shared=121, delta=120, outputs=3
//!   fxc: shared=241, delta=183, outputs=6
//!   kxc: shared=424, delta=212, outputs=10
//!   lxc: shared=636, delta=153, outputs=15
//! pol: preamble=209 lines
//!   exc: shared=0, delta=209, outputs=1
//!   vxc: shared=209, delta=318, outputs=6
//!   fxc: shared=527, delta=743, outputs=21
//!   kxc: shared=1270, delta=1553, outputs=56
//!   lxc: shared=2823, delta=1880, outputs=126

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
