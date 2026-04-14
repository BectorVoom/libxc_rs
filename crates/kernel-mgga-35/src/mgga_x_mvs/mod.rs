//! MGGA_X_MVS kernel -- incremental derivative structure.

//! unpol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=52, outputs=5
//!   fxc: shared=103, delta=121, outputs=15
//!   kxc: shared=224, delta=227, outputs=35
//!   lxc: shared=451, delta=199, outputs=70
//! pol: preamble=91 lines
//!   exc: shared=0, delta=91, outputs=1
//!   vxc: shared=91, delta=114, outputs=10
//!   fxc: shared=205, delta=322, outputs=55
//!   kxc: shared=527, delta=762, outputs=220
//!   lxc: shared=1289, delta=1381, outputs=715

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
