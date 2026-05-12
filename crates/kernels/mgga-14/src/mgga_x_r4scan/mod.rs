//! MGGA_X_R4SCAN kernel -- incremental derivative structure.

//! unpol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=163, outputs=5
//!   fxc: shared=279, delta=527, outputs=15
//!   kxc: shared=806, delta=1507, outputs=35
//!   lxc: shared=2313, delta=1740, outputs=70
//! pol: preamble=199 lines
//!   exc: shared=0, delta=199, outputs=1
//!   vxc: shared=199, delta=341, outputs=10
//!   fxc: shared=540, delta=1126, outputs=55
//!   kxc: shared=1666, delta=3383, outputs=220
//!   lxc: shared=5049, delta=4247, outputs=715

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
pub mod lxc_pol_part1_v4rho4_v4rho3sigma_v4rho3lapl_v4rho3tau;
pub mod lxc_pol_part2_v4rho2sigma2_v4rho2sigmalapl;
pub mod lxc_pol_part3_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
pub mod lxc_pol_part4_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhosigmalapl2_v_etc;
pub mod lxc_pol_part5_v4rhosigmatau2_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhot_etc;
