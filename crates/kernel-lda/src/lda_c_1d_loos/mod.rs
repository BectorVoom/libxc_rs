//! LDA_C_1D_LOOS kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=21 lines
//!   exc: shared=0, delta=21, outputs=1
//!   vxc: shared=21, delta=10, outputs=2
//!   fxc: shared=31, delta=18, outputs=3
//!   kxc: shared=49, delta=25, outputs=4
//!   lxc: shared=74, delta=13, outputs=5
//! pol: preamble=22 lines
//!   exc: shared=0, delta=22, outputs=1
//!   vxc: shared=22, delta=11, outputs=3
//!   fxc: shared=33, delta=20, outputs=6
//!   kxc: shared=53, delta=27, outputs=10
//!   lxc: shared=80, delta=17, outputs=15

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
