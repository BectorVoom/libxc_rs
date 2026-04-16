//! MGGA_X_MGGAC kernel -- incremental derivative structure.

//! unpol: preamble=94 lines
//!   exc: shared=0, delta=94, outputs=1
//!   vxc: shared=94, delta=134, outputs=5
//!   fxc: shared=228, delta=579, outputs=15
//!   kxc: shared=807, delta=2415, outputs=35
//!   lxc: shared=3222, delta=7086, outputs=70
//! pol: preamble=160 lines
//!   exc: shared=0, delta=160, outputs=1
//!   vxc: shared=160, delta=291, outputs=10
//!   fxc: shared=451, delta=1243, outputs=55
//!   kxc: shared=1694, delta=5115, outputs=220
//!   lxc: shared=6809, delta=14829, outputs=715

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
