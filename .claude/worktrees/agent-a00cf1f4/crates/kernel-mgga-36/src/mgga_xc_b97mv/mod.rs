//! MGGA_XC_B97MV kernel -- incremental derivative structure.

//! unpol: preamble=176 lines
//!   exc: shared=0, delta=176, outputs=1
//!   vxc: shared=176, delta=215, outputs=5
//!   fxc: shared=391, delta=411, outputs=15
//!   kxc: shared=802, delta=776, outputs=35
//!   lxc: shared=1578, delta=580, outputs=70
//! pol: preamble=281 lines
//!   exc: shared=0, delta=281, outputs=1
//!   vxc: shared=281, delta=508, outputs=10
//!   fxc: shared=789, delta=1343, outputs=55
//!   kxc: shared=2132, delta=3358, outputs=220
//!   lxc: shared=5490, delta=5012, outputs=715

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
