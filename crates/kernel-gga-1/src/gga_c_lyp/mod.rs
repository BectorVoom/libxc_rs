//! GGA_C_LYP kernel -- incremental derivative structure.

//! unpol: preamble=38 lines
//!   exc: shared=0, delta=38, outputs=1
//!   vxc: shared=38, delta=34, outputs=3
//!   fxc: shared=72, delta=43, outputs=6
//!   kxc: shared=115, delta=59, outputs=10
//!   lxc: shared=174, delta=23, outputs=15
//! pol: preamble=78 lines
//!   exc: shared=0, delta=78, outputs=1
//!   vxc: shared=78, delta=120, outputs=6
//!   fxc: shared=198, delta=269, outputs=21
//!   kxc: shared=467, delta=550, outputs=56
//!   lxc: shared=1017, delta=673, outputs=126

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
