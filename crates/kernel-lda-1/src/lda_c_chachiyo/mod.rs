//! LDA_C_CHACHIYO kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=9, outputs=2
//!   fxc: shared=37, delta=16, outputs=3
//!   kxc: shared=53, delta=16, outputs=4
//!   lxc: shared=69, delta=15, outputs=5
//! pol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=26, outputs=3
//!   fxc: shared=67, delta=56, outputs=6
//!   kxc: shared=123, delta=72, outputs=10
//!   lxc: shared=195, delta=68, outputs=15

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
