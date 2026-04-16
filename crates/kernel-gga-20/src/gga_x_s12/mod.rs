//! GGA_X_S12 kernel -- incremental derivative structure.

//! unpol: preamble=34 lines
//!   exc: shared=0, delta=34, outputs=1
//!   vxc: shared=34, delta=26, outputs=3
//!   fxc: shared=60, delta=40, outputs=6
//!   kxc: shared=100, delta=72, outputs=10
//!   lxc: shared=172, delta=52, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=73, outputs=6
//!   fxc: shared=139, delta=131, outputs=21
//!   kxc: shared=270, delta=264, outputs=56
//!   lxc: shared=534, delta=307, outputs=126

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
