//! LDA_XC_TIH kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=9 lines
//!   vxc: shared=0, delta=9, outputs=1
//!   fxc: shared=9, delta=9, outputs=2
//!   kxc: shared=18, delta=9, outputs=3
//!   lxc: shared=27, delta=9, outputs=4
//! pol: preamble=10 lines
//!   vxc: shared=0, delta=10, outputs=2
//!   fxc: shared=10, delta=11, outputs=5
//!   kxc: shared=21, delta=12, outputs=9
//!   lxc: shared=33, delta=13, outputs=14

pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
