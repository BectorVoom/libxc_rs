//! LDA_K_TF kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=14 lines
//!   exc: shared=0, delta=14, outputs=1
//!   vxc: shared=14, delta=1, outputs=2
//!   fxc: shared=15, delta=1, outputs=3
//!   kxc: shared=16, delta=1, outputs=4
//!   lxc: shared=17, delta=2, outputs=5
//! pol: preamble=30 lines
//!   exc: shared=0, delta=30, outputs=1
//!   vxc: shared=30, delta=18, outputs=3
//!   fxc: shared=48, delta=35, outputs=6
//!   kxc: shared=83, delta=51, outputs=10
//!   lxc: shared=134, delta=47, outputs=15

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
