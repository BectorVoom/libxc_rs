//! GGA_C_ACGGA kernel -- incremental derivative structure.

//! unpol: preamble=91 lines
//!   exc: shared=0, delta=91, outputs=1
//!   vxc: shared=91, delta=130, outputs=3
//!   fxc: shared=221, delta=269, outputs=6
//!   kxc: shared=490, delta=519, outputs=10
//!   lxc: shared=1009, delta=342, outputs=15
//! pol: preamble=126 lines
//!   exc: shared=0, delta=126, outputs=1
//!   vxc: shared=126, delta=231, outputs=6
//!   fxc: shared=357, delta=840, outputs=21
//!   kxc: shared=1197, delta=3075, outputs=56
//!   lxc: shared=4272, delta=6729, outputs=126

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
