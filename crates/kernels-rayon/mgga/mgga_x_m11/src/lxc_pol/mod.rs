//! MGGA_X_M11 lxc pol kernel — lxc_pol (nested-by-output, 7 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::mgga_x_m11_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_m11_lxc_pol_part1_v3rho3_v3rho2sigma_v3rho2lapl;
use part2::mgga_x_m11_lxc_pol_part2_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc;
use part3::mgga_x_m11_lxc_pol_part3_v4rho4;
use part4::mgga_x_m11_lxc_pol_part4_v4rho3sigma_v4rho3lapl;
use part5::mgga_x_m11_lxc_pol_part5_v4rho3tau_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho_etc;
use part6::mgga_x_m11_lxc_pol_part6_v4rho2tau2_5_v4rho2tau2_6_v4rho2tau2_7_v4rho2tau2_8_v4rhosig_etc;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_m11_lxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho3lapl: &mut [f64],
    v4rho3tau: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rho2sigmalapl: &mut [f64],
    v4rho2sigmatau: &mut [f64],
    v4rho2lapl2: &mut [f64],
    v4rho2lapltau: &mut [f64],
    v4rho2tau2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4rhosigma2lapl: &mut [f64],
    v4rhosigma2tau: &mut [f64],
    v4rhosigmalapl2: &mut [f64],
    v4rhosigmalapltau: &mut [f64],
    v4rhosigmatau2: &mut [f64],
    v4rholapl3: &mut [f64],
    v4rholapl2tau: &mut [f64],
    v4rholapltau2: &mut [f64],
    v4rhotau3: &mut [f64],
    v4sigma4: &mut [f64],
    v4sigma3lapl: &mut [f64],
    v4sigma3tau: &mut [f64],
    v4sigma2lapl2: &mut [f64],
    v4sigma2lapltau: &mut [f64],
    v4sigma2tau2: &mut [f64],
    v4sigmalapl3: &mut [f64],
    v4sigmalapl2tau: &mut [f64],
    v4sigmalapltau2: &mut [f64],
    v4sigmatau3: &mut [f64],
    v4lapl4: &mut [f64],
    v4lapl3tau: &mut [f64],
    v4lapl2tau2: &mut [f64],
    v4lapltau3: &mut [f64],
    v4tau4: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_m11_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part1_v3rho3_v3rho2sigma_v3rho2lapl(rho, sigma, lapl, tau, v3rho3, v3rho2sigma, v3rho2lapl, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part2_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part3_v4rho4(rho, sigma, lapl, tau, v4rho4, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part4_v4rho3sigma_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part5_v4rho3tau_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho_etc(rho, sigma, lapl, tau, v4rho3tau, v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
    mgga_x_m11_lxc_pol_part6_v4rho2tau2_5_v4rho2tau2_6_v4rho2tau2_7_v4rho2tau2_8_v4rhosig_etc(rho, sigma, lapl, tau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_a_6, param_a_7, param_a_8, param_a_9, param_a_10, param_a_11, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_b_9, param_b_10, param_b_11, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
