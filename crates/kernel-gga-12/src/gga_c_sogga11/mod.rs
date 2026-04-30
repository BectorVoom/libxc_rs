//! GGA_C_SOGGA11 kernel -- incremental derivative structure.

//! unpol: preamble=64 lines
//!   exc: shared=0, delta=64, outputs=1
//!   vxc: shared=64, delta=74, outputs=3
//!   fxc: shared=138, delta=169, outputs=6
//!   kxc: shared=307, delta=312, outputs=10
//!   lxc: shared=619, delta=267, outputs=15
//! pol: preamble=97 lines
//!   exc: shared=0, delta=97, outputs=1
//!   vxc: shared=97, delta=149, outputs=6
//!   fxc: shared=246, delta=558, outputs=21
//!   kxc: shared=804, delta=2011, outputs=56
//!   lxc: shared=2815, delta=6838, outputs=126

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
