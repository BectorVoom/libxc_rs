//! LDA_C_RC04 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=18 lines
//!   exc: shared=0, delta=18, outputs=1
//!   vxc: shared=18, delta=4, outputs=2
//!   fxc: shared=22, delta=5, outputs=3
//!   kxc: shared=27, delta=9, outputs=4
//!   lxc: shared=36, delta=6, outputs=5
//! pol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=26, outputs=3
//!   fxc: shared=61, delta=59, outputs=6
//!   kxc: shared=120, delta=112, outputs=10
//!   lxc: shared=232, delta=134, outputs=15

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
