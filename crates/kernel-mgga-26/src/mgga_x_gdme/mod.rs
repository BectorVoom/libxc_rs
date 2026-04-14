//! MGGA_X_GDME kernel -- incremental derivative structure.

//! unpol: preamble=33 lines
//!   exc: shared=0, delta=33, outputs=1
//!   vxc: shared=33, delta=18, outputs=5
//!   fxc: shared=51, delta=19, outputs=15
//!   kxc: shared=70, delta=29, outputs=35
//!   lxc: shared=99, delta=40, outputs=70
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=56, outputs=10
//!   fxc: shared=113, delta=124, outputs=55
//!   kxc: shared=237, delta=307, outputs=220
//!   lxc: shared=544, delta=684, outputs=715

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
