//! LDA_C_HL kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=44 lines
//!   exc: shared=0, delta=44, outputs=1
//!   vxc: shared=44, delta=16, outputs=2
//!   fxc: shared=60, delta=21, outputs=3
//!   kxc: shared=81, delta=19, outputs=4
//!   lxc: shared=100, delta=14, outputs=5
//! pol: preamble=58 lines
//!   exc: shared=0, delta=58, outputs=1
//!   vxc: shared=58, delta=31, outputs=3
//!   fxc: shared=89, delta=59, outputs=6
//!   kxc: shared=148, delta=73, outputs=10
//!   lxc: shared=221, delta=63, outputs=15

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
