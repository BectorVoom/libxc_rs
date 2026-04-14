//! MGGA_XC_B98 kernel -- incremental derivative structure.

//! unpol: preamble=102 lines
//!   exc: shared=0, delta=102, outputs=1
//!   vxc: shared=102, delta=179, outputs=5
//!   fxc: shared=281, delta=424, outputs=15
//!   kxc: shared=705, delta=920, outputs=35
//!   lxc: shared=1625, delta=879, outputs=70
//! pol: preamble=195 lines
//!   exc: shared=0, delta=195, outputs=1
//!   vxc: shared=195, delta=458, outputs=10
//!   fxc: shared=653, delta=1402, outputs=55
//!   kxc: shared=2055, delta=4106, outputs=220
//!   lxc: shared=6161, delta=5367, outputs=715

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
