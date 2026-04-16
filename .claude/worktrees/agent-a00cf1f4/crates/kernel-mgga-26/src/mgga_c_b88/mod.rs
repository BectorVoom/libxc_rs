//! MGGA_C_B88 kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=83, outputs=5
//!   fxc: shared=153, delta=172, outputs=15
//!   kxc: shared=325, delta=336, outputs=35
//!   lxc: shared=661, delta=212, outputs=70
//! pol: preamble=132 lines
//!   exc: shared=0, delta=132, outputs=1
//!   vxc: shared=132, delta=226, outputs=10
//!   fxc: shared=358, delta=775, outputs=55
//!   kxc: shared=1133, delta=2632, outputs=220
//!   lxc: shared=3765, delta=3632, outputs=715

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
