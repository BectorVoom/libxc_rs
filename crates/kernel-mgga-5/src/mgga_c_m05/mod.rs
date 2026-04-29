//! MGGA_C_M05 kernel -- incremental derivative structure.

//! unpol: preamble=145 lines
//!   exc: shared=0, delta=145, outputs=1
//!   vxc: shared=145, delta=161, outputs=5
//!   fxc: shared=306, delta=286, outputs=15
//!   kxc: shared=592, delta=462, outputs=35
//!   lxc: shared=1054, delta=251, outputs=70
//! pol: preamble=227 lines
//!   exc: shared=0, delta=227, outputs=1
//!   vxc: shared=227, delta=371, outputs=10
//!   fxc: shared=598, delta=1017, outputs=55
//!   kxc: shared=1615, delta=2439, outputs=220
//!   lxc: shared=4054, delta=2720, outputs=715

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
