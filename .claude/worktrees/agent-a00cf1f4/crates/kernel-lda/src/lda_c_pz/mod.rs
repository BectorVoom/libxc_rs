//! LDA_C_PZ kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=17, outputs=2
//!   fxc: shared=58, delta=23, outputs=3
//!   kxc: shared=81, delta=22, outputs=4
//!   lxc: shared=103, delta=15, outputs=5
//! pol: preamble=54 lines
//!   exc: shared=0, delta=54, outputs=1
//!   vxc: shared=54, delta=33, outputs=3
//!   fxc: shared=87, delta=61, outputs=6
//!   kxc: shared=148, delta=74, outputs=10
//!   lxc: shared=222, delta=64, outputs=15

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
