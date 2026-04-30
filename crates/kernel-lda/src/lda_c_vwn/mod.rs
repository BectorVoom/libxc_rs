//! LDA_C_VWN kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=51, outputs=2
//!   fxc: shared=92, delta=92, outputs=3
//!   kxc: shared=184, delta=148, outputs=4
//!   lxc: shared=332, delta=73, outputs=5
//! pol: preamble=76 lines
//!   exc: shared=0, delta=76, outputs=1
//!   vxc: shared=76, delta=118, outputs=3
//!   fxc: shared=194, delta=249, outputs=6
//!   kxc: shared=443, delta=461, outputs=10
//!   lxc: shared=904, delta=430, outputs=15

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
