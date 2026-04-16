//! LDA_C_PW kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=50 lines
//!   exc: shared=0, delta=50, outputs=1
//!   vxc: shared=50, delta=28, outputs=2
//!   fxc: shared=78, delta=59, outputs=3
//!   kxc: shared=137, delta=81, outputs=4
//!   lxc: shared=218, delta=28, outputs=5
//! pol: preamble=90 lines
//!   exc: shared=0, delta=90, outputs=1
//!   vxc: shared=90, delta=79, outputs=3
//!   fxc: shared=169, delta=195, outputs=6
//!   kxc: shared=364, delta=345, outputs=10
//!   lxc: shared=709, delta=361, outputs=15

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
