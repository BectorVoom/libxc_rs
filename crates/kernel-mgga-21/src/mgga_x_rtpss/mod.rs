//! MGGA_X_RTPSS kernel -- incremental derivative structure.

//! unpol: preamble=82 lines
//!   exc: shared=0, delta=82, outputs=1
//!   vxc: shared=82, delta=97, outputs=5
//!   fxc: shared=179, delta=191, outputs=15
//!   kxc: shared=370, delta=390, outputs=35
//!   lxc: shared=760, delta=336, outputs=70
//! pol: preamble=141 lines
//!   exc: shared=0, delta=141, outputs=1
//!   vxc: shared=141, delta=209, outputs=10
//!   fxc: shared=350, delta=493, outputs=55
//!   kxc: shared=843, delta=1106, outputs=220
//!   lxc: shared=1949, delta=1497, outputs=715

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
