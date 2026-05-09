//! LDA_C_LP96 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=5 lines
//!   exc: shared=0, delta=5, outputs=1
//!   vxc: shared=5, delta=3, outputs=2
//!   fxc: shared=8, delta=4, outputs=3
//!   kxc: shared=12, delta=4, outputs=4
//!   lxc: shared=16, delta=2, outputs=5
//! pol: preamble=6 lines
//!   exc: shared=0, delta=6, outputs=1
//!   vxc: shared=6, delta=4, outputs=3
//!   fxc: shared=10, delta=6, outputs=6
//!   kxc: shared=16, delta=7, outputs=10
//!   lxc: shared=23, delta=6, outputs=15

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
