//! GGA_X_FD_LB94 kernel -- incremental derivative structure.

//! unpol: preamble=33 lines
//!   exc: shared=0, delta=33, outputs=1
//!   vxc: shared=33, delta=16, outputs=3
//!   fxc: shared=49, delta=46, outputs=6
//!   kxc: shared=95, delta=44, outputs=10
//!   lxc: shared=139, delta=32, outputs=15
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=47, outputs=6
//!   fxc: shared=104, delta=145, outputs=21
//!   kxc: shared=249, delta=218, outputs=56
//!   lxc: shared=467, delta=273, outputs=126

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
