//! GGA_C_OP_PW91 kernel -- incremental derivative structure.

//! unpol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=57, outputs=3
//!   fxc: shared=142, delta=118, outputs=6
//!   kxc: shared=260, delta=207, outputs=10
//!   lxc: shared=467, delta=183, outputs=15
//! pol: preamble=112 lines
//!   exc: shared=0, delta=112, outputs=1
//!   vxc: shared=112, delta=126, outputs=6
//!   fxc: shared=238, delta=369, outputs=21
//!   kxc: shared=607, delta=1008, outputs=56
//!   lxc: shared=1615, delta=1691, outputs=126

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
