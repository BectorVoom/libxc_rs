//! GGA_K_OL1 kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=13, outputs=3
//!   fxc: shared=45, delta=14, outputs=6
//!   kxc: shared=59, delta=16, outputs=10
//!   lxc: shared=75, delta=10, outputs=15
//! pol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=45, outputs=6
//!   fxc: shared=101, delta=93, outputs=21
//!   kxc: shared=194, delta=176, outputs=56
//!   lxc: shared=370, delta=236, outputs=126

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
