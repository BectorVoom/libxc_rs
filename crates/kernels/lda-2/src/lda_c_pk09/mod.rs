//! LDA_C_PK09 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=199 lines
//!   exc: shared=0, delta=199, outputs=1
//!   vxc: shared=199, delta=335, outputs=2
//!   fxc: shared=534, delta=550, outputs=3
//!   kxc: shared=1084, delta=304, outputs=4
//! pol: preamble=363 lines
//!   exc: shared=0, delta=363, outputs=1
//!   vxc: shared=363, delta=1260, outputs=3
//!   fxc: shared=1623, delta=4368, outputs=6
//!   kxc: shared=5991, delta=11519, outputs=10

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol_part0_zk_vrho_v2rho2_0;
pub mod fxc_pol_part1_v2rho2_1;
pub mod fxc_pol_part2_v2rho2_2;
pub mod kxc_pol_part0_zk_vrho_v2rho2_0;
pub mod kxc_pol_part1_v2rho2_1;
pub mod kxc_pol_part2_v2rho2_2;
pub mod kxc_pol_part3_v3rho3_0;
pub mod kxc_pol_part4_v3rho3_1;
pub mod kxc_pol_part5_v3rho3_2;
pub mod kxc_pol_part6_v3rho3_3;
