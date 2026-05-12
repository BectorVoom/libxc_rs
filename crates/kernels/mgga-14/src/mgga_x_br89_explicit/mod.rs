//! MGGA_X_BR89_EXPLICIT kernel -- incremental derivative structure.

//! unpol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=163, outputs=5
//!   fxc: shared=235, delta=658, outputs=15
//!   kxc: shared=893, delta=2461, outputs=35
//!   lxc: shared=3354, delta=5357, outputs=70
//! pol: preamble=141 lines
//!   exc: shared=0, delta=141, outputs=1
//!   vxc: shared=141, delta=348, outputs=10
//!   fxc: shared=489, delta=1430, outputs=55
//!   kxc: shared=1919, delta=5489, outputs=220
//!   lxc: shared=7408, delta=13444, outputs=715

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
pub mod lxc_unpol_part1_v4rho2lapltau_v4rho2tau2_v4rhosigma3_v4rhosigma2lapl_v4rhosi_etc;
pub mod lxc_unpol_part2_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4_v4sigma3lapl__etc;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
pub mod kxc_pol_part1_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
pub mod lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
pub mod lxc_pol_part1_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
pub mod lxc_pol_part2_v4rho3sigma_v4rho3lapl_v4rho3tau;
pub mod lxc_pol_part3_v4rho2sigma2_v4rho2sigmalapl;
pub mod lxc_pol_part4_v4rho2sigmatau_v4rho2lapl2;
pub mod lxc_pol_part5_v4rho2lapltau_v4rho2tau2_v4rhosigma3;
pub mod lxc_pol_part6_v4rhosigma2lapl_v4rhosigma2tau;
pub mod lxc_pol_part7_v4rhosigmalapl2;
pub mod lxc_pol_part8_v4rhosigmalapltau;
pub mod lxc_pol_part9_v4rhosigmatau2_v4rholapl3;
pub mod lxc_pol_part10_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4;
pub mod lxc_pol_part11_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau_v4sig_etc;
pub mod lxc_pol_part12_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3_v4l_etc;
pub mod lxc_pol_part13_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;
