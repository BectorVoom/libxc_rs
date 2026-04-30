//! GGA_X_C09X kernel -- incremental derivative structure.

//! unpol: preamble=34 lines
//!   exc: shared=0, delta=34, outputs=1
//!   vxc: shared=34, delta=25, outputs=3
//!   fxc: shared=59, delta=27, outputs=6
//!   kxc: shared=86, delta=33, outputs=10
//!   lxc: shared=119, delta=25, outputs=15
//! pol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=57, outputs=6
//!   fxc: shared=112, delta=108, outputs=21
//!   kxc: shared=220, delta=193, outputs=56
//!   lxc: shared=413, delta=259, outputs=126

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
