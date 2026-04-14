//! LDA_C_VWN_2 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=69 lines
//!   exc: shared=0, delta=69, outputs=1
//!   vxc: shared=69, delta=107, outputs=2
//!   fxc: shared=176, delta=196, outputs=3
//!   kxc: shared=372, delta=329, outputs=4
//!   lxc: shared=701, delta=198, outputs=5
//! pol: preamble=93 lines
//!   exc: shared=0, delta=93, outputs=1
//!   vxc: shared=93, delta=155, outputs=3
//!   fxc: shared=248, delta=313, outputs=6
//!   kxc: shared=561, delta=557, outputs=10
//!   lxc: shared=1118, delta=425, outputs=15

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
