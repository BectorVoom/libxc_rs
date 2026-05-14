//! MGGA_C_M08 lxc pol kernel — lxc_pol (nested-by-output, 8 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_m08_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_m08_lxc_pol_part1_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc;
use part2::mgga_c_m08_lxc_pol_part2_v4rho4;
use part3::mgga_c_m08_lxc_pol_part3_v4rho3sigma_v4rho3lapl;
use part4::mgga_c_m08_lxc_pol_part4_v4rho3tau;
use part5::mgga_c_m08_lxc_pol_part5_v4rho2sigma2_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2_v4rh_etc;
use part6::mgga_c_m08_lxc_pol_part6_v4rho2tau2_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhos_etc;
use part7::mgga_c_m08_lxc_pol_part7_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_m08_lxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho3lapl: &mut Array<f64>,
    v4rho3tau: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rho2sigmalapl: &mut Array<f64>,
    v4rho2sigmatau: &mut Array<f64>,
    v4rho2lapl2: &mut Array<f64>,
    v4rho2lapltau: &mut Array<f64>,
    v4rho2tau2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4rhosigma2lapl: &mut Array<f64>,
    v4rhosigma2tau: &mut Array<f64>,
    v4rhosigmalapl2: &mut Array<f64>,
    v4rhosigmalapltau: &mut Array<f64>,
    v4rhosigmatau2: &mut Array<f64>,
    v4rholapl3: &mut Array<f64>,
    v4rholapl2tau: &mut Array<f64>,
    v4rholapltau2: &mut Array<f64>,
    v4rhotau3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    v4sigma3lapl: &mut Array<f64>,
    v4sigma3tau: &mut Array<f64>,
    v4sigma2lapl2: &mut Array<f64>,
    v4sigma2lapltau: &mut Array<f64>,
    v4sigma2tau2: &mut Array<f64>,
    v4sigmalapl3: &mut Array<f64>,
    v4sigmalapl2tau: &mut Array<f64>,
    v4sigmalapltau2: &mut Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_m08_a_0: f64,
    param_m08_a_1: f64,
    param_m08_a_2: f64,
    param_m08_a_3: f64,
    param_m08_a_4: f64,
    param_m08_a_5: f64,
    param_m08_a_6: f64,
    param_m08_a_7: f64,
    param_m08_a_8: f64,
    param_m08_a_9: f64,
    param_m08_a_10: f64,
    param_m08_a_11: f64,
    param_m08_b_0: f64,
    param_m08_b_1: f64,
    param_m08_b_2: f64,
    param_m08_b_3: f64,
    param_m08_b_4: f64,
    param_m08_b_5: f64,
    param_m08_b_6: f64,
    param_m08_b_7: f64,
    param_m08_b_8: f64,
    param_m08_b_9: f64,
    param_m08_b_10: f64,
    param_m08_b_11: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_m08_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, v3rho2sigma, v3rho2lapl, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part1_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part2_v4rho4(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part3_v4rho3sigma_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part4_v4rho3tau(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part5_v4rho2sigma2_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2_v4rh_etc(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part6_v4rho2tau2_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhos_etc(rho, sigma, lapl, tau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part7_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4_etc(rho, sigma, lapl, tau, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
}
