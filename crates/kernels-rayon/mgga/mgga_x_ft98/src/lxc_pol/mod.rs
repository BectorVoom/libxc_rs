//! MGGA_X_FT98 lxc pol kernel — lxc_pol (nested-by-output, 17 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;
mod part8;
mod part9;
mod part10;
mod part11;
mod part12;
mod part13;
mod part14;
mod part15;
mod part16;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

use part0::mgga_x_ft98_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_ft98_lxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau;
use part2::mgga_x_ft98_lxc_pol_part2_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau;
use part3::mgga_x_ft98_lxc_pol_part3_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3sigma2lapl_v3si_etc;
use part4::mgga_x_ft98_lxc_pol_part4_v4rho4;
use part5::mgga_x_ft98_lxc_pol_part5_v4rho3sigma;
use part6::mgga_x_ft98_lxc_pol_part6_v4rho3lapl_v4rho3tau;
use part7::mgga_x_ft98_lxc_pol_part7_v4rho2sigma2;
use part8::mgga_x_ft98_lxc_pol_part8_v4rho2sigmalapl_v4rho2sigmatau;
use part9::mgga_x_ft98_lxc_pol_part9_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
use part10::mgga_x_ft98_lxc_pol_part10_v4rhosigma3;
use part11::mgga_x_ft98_lxc_pol_part11_v4rhosigma2lapl_v4rhosigma2tau;
use part12::mgga_x_ft98_lxc_pol_part12_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2;
use part13::mgga_x_ft98_lxc_pol_part13_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4;
use part14::mgga_x_ft98_lxc_pol_part14_v4sigma3lapl_v4sigma3tau;
use part15::mgga_x_ft98_lxc_pol_part15_v4sigma2lapl2_v4sigma2lapltau_v4sigma2tau2;
use part16::mgga_x_ft98_lxc_pol_part16_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3_v4l_etc;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_ft98_lxc_pol(
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
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_ft98_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, v3rho2tau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part2_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part3_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3sigma2lapl_v3si_etc(rho, sigma, lapl, tau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part4_v4rho4(rho, sigma, lapl, tau, v4rho4, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part5_v4rho3sigma(rho, sigma, lapl, tau, v4rho3sigma, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part6_v4rho3lapl_v4rho3tau(rho, sigma, lapl, tau, v4rho3lapl, v4rho3tau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part7_v4rho2sigma2(rho, sigma, lapl, tau, v4rho2sigma2, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part8_v4rho2sigmalapl_v4rho2sigmatau(rho, sigma, lapl, tau, v4rho2sigmalapl, v4rho2sigmatau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part9_v4rho2lapl2_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part10_v4rhosigma3(rho, sigma, lapl, tau, v4rhosigma3, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part11_v4rhosigma2lapl_v4rhosigma2tau(rho, sigma, lapl, tau, v4rhosigma2lapl, v4rhosigma2tau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part12_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2(rho, sigma, lapl, tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part13_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4(rho, sigma, lapl, tau, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, v4sigma4, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part14_v4sigma3lapl_v4sigma3tau(rho, sigma, lapl, tau, v4sigma3lapl, v4sigma3tau, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part15_v4sigma2lapl2_v4sigma2lapltau_v4sigma2tau2(rho, sigma, lapl, tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
    mgga_x_ft98_lxc_pol_part16_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3_v4l_etc(rho, sigma, lapl, tau, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_a, param_a1, param_a2, param_b, param_b1, param_b2, dens_threshold, zeta_threshold);
}
