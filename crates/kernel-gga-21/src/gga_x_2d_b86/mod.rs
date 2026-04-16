//! GGA_X_2D_B86 kernel -- incremental derivative structure.

//! unpol: preamble=20 lines
//!   exc: shared=0, delta=20, outputs=1
//!   vxc: shared=20, delta=11, outputs=3
//!   fxc: shared=31, delta=24, outputs=6
//!   kxc: shared=55, delta=27, outputs=10
//!   lxc: shared=82, delta=18, outputs=15
//! pol: preamble=49 lines
//!   exc: shared=0, delta=49, outputs=1
//!   vxc: shared=49, delta=56, outputs=6
//!   fxc: shared=105, delta=138, outputs=21
//!   kxc: shared=243, delta=261, outputs=56
//!   lxc: shared=504, delta=398, outputs=126

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
