//! LDA_C_W20 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=90 lines
//!   exc: shared=0, delta=90, outputs=1
//!   vxc: shared=90, delta=73, outputs=2
//!   fxc: shared=163, delta=136, outputs=3
//!   kxc: shared=299, delta=231, outputs=4
//!   lxc: shared=530, delta=155, outputs=5
//! pol: preamble=103 lines
//!   exc: shared=0, delta=103, outputs=1
//!   vxc: shared=103, delta=87, outputs=3
//!   fxc: shared=190, delta=179, outputs=6
//!   kxc: shared=369, delta=304, outputs=10
//!   lxc: shared=673, delta=252, outputs=15

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
