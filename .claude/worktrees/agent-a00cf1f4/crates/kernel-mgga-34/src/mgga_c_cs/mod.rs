//! MGGA_C_CS kernel -- incremental derivative structure.

//! unpol: preamble=21 lines
//!   exc: shared=0, delta=21, outputs=1
//!   vxc: shared=21, delta=18, outputs=5
//!   fxc: shared=39, delta=29, outputs=15
//!   kxc: shared=68, delta=46, outputs=35
//!   lxc: shared=114, delta=39, outputs=70
//! pol: preamble=57 lines
//!   exc: shared=0, delta=57, outputs=1
//!   vxc: shared=57, delta=82, outputs=10
//!   fxc: shared=139, delta=216, outputs=55
//!   kxc: shared=355, delta=567, outputs=220
//!   lxc: shared=922, delta=1162, outputs=715

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
