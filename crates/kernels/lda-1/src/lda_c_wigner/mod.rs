//! LDA_C_WIGNER kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=10 lines
//!   exc: shared=0, delta=10, outputs=1
//!   vxc: shared=10, delta=3, outputs=2
//!   fxc: shared=13, delta=7, outputs=3
//!   kxc: shared=20, delta=7, outputs=4
//!   lxc: shared=27, delta=2, outputs=5
//! pol: preamble=18 lines
//!   exc: shared=0, delta=18, outputs=1
//!   vxc: shared=18, delta=14, outputs=3
//!   fxc: shared=32, delta=27, outputs=6
//!   kxc: shared=59, delta=40, outputs=10
//!   lxc: shared=99, delta=37, outputs=15

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
