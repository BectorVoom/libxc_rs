//! GGA_C_SCAN_E0 kernel -- incremental derivative structure.

//! unpol: preamble=64 lines
//!   exc: shared=0, delta=64, outputs=1
//!   vxc: shared=64, delta=75, outputs=3
//!   fxc: shared=139, delta=168, outputs=6
//!   kxc: shared=307, delta=293, outputs=10
//!   lxc: shared=600, delta=242, outputs=15
//! pol: preamble=99 lines
//!   exc: shared=0, delta=99, outputs=1
//!   vxc: shared=99, delta=153, outputs=6
//!   fxc: shared=252, delta=452, outputs=21
//!   kxc: shared=704, delta=1233, outputs=56
//!   lxc: shared=1937, delta=2421, outputs=126

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
