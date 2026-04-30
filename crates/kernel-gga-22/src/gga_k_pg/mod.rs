//! GGA_K_PG kernel -- incremental derivative structure.

//! unpol: preamble=34 lines
//!   exc: shared=0, delta=34, outputs=1
//!   vxc: shared=34, delta=10, outputs=3
//!   fxc: shared=44, delta=23, outputs=6
//!   kxc: shared=67, delta=27, outputs=10
//!   lxc: shared=94, delta=19, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=43, outputs=6
//!   fxc: shared=100, delta=103, outputs=21
//!   kxc: shared=203, delta=202, outputs=56
//!   lxc: shared=405, delta=244, outputs=126

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
