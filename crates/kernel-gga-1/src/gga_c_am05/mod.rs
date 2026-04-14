//! GGA_C_AM05 kernel -- incremental derivative structure.

//! unpol: preamble=45 lines
//!   exc: shared=0, delta=45, outputs=1
//!   vxc: shared=45, delta=47, outputs=3
//!   fxc: shared=92, delta=78, outputs=6
//!   kxc: shared=170, delta=86, outputs=10
//!   lxc: shared=256, delta=54, outputs=15
//! pol: preamble=82 lines
//!   exc: shared=0, delta=82, outputs=1
//!   vxc: shared=82, delta=100, outputs=6
//!   fxc: shared=182, delta=225, outputs=21
//!   kxc: shared=407, delta=396, outputs=56
//!   lxc: shared=803, delta=496, outputs=126

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
