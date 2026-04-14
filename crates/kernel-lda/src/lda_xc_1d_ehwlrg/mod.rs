//! LDA_XC_1D_EHWLRG kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=4 lines
//!   exc: shared=0, delta=4, outputs=1
//!   vxc: shared=4, delta=3, outputs=2
//!   fxc: shared=7, delta=6, outputs=3
//!   kxc: shared=13, delta=6, outputs=4
//!   lxc: shared=19, delta=3, outputs=5
//! pol: preamble=5 lines
//!   exc: shared=0, delta=5, outputs=1
//!   vxc: shared=5, delta=4, outputs=3
//!   fxc: shared=9, delta=8, outputs=6
//!   kxc: shared=17, delta=9, outputs=10
//!   lxc: shared=26, delta=7, outputs=15

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
