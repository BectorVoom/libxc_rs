//! GGA_C_W94 kernel -- incremental derivative structure.

//! unpol: preamble=20 lines
//!   exc: shared=0, delta=20, outputs=1
//!   vxc: shared=20, delta=14, outputs=3
//!   fxc: shared=34, delta=22, outputs=6
//!   kxc: shared=56, delta=28, outputs=10
//!   lxc: shared=84, delta=17, outputs=15
//! pol: preamble=34 lines
//!   exc: shared=0, delta=34, outputs=1
//!   vxc: shared=34, delta=31, outputs=6
//!   fxc: shared=65, delta=92, outputs=21
//!   kxc: shared=157, delta=287, outputs=56
//!   lxc: shared=444, delta=635, outputs=126

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
