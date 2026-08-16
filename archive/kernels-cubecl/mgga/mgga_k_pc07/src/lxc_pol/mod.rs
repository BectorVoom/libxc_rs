//! MGGA_K_PC07 lxc pol kernel — lxc_pol (nested-by-output, 21 parts).
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
mod part17;
mod part18;
mod part19;
mod part20;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

use part0::mgga_k_pc07_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_k_pc07_lxc_pol_part1_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc;
use part2::mgga_k_pc07_lxc_pol_part2_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigma_etc;
use part3::mgga_k_pc07_lxc_pol_part3_v4rho4;
use part4::mgga_k_pc07_lxc_pol_part4_v4rho3sigma;
use part5::mgga_k_pc07_lxc_pol_part5_v4rho3lapl_v4rho3tau;
use part6::mgga_k_pc07_lxc_pol_part6_v4rho2sigma2;
use part7::mgga_k_pc07_lxc_pol_part7_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc;
use part8::mgga_k_pc07_lxc_pol_part8_v4rho2sigmalapl_17;
use part9::mgga_k_pc07_lxc_pol_part9_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
use part10::mgga_k_pc07_lxc_pol_part10_v4rhosigma3;
use part11::mgga_k_pc07_lxc_pol_part11_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc;
use part12::mgga_k_pc07_lxc_pol_part12_v4rhosigma2lapl_23_v4rhosigma2tau;
use part13::mgga_k_pc07_lxc_pol_part13_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc;
use part14::mgga_k_pc07_lxc_pol_part14_v4rhosigmalapl2_17;
use part15::mgga_k_pc07_lxc_pol_part15_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3_v4rholapl2tau_v4_etc;
use part16::mgga_k_pc07_lxc_pol_part16_v4sigma4;
use part17::mgga_k_pc07_lxc_pol_part17_v4sigma3lapl_v4sigma3tau;
use part18::mgga_k_pc07_lxc_pol_part18_v4sigma2lapl2_v4sigma2lapltau_v4sigma2tau2;
use part19::mgga_k_pc07_lxc_pol_part19_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3;
use part20::mgga_k_pc07_lxc_pol_part20_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_pc07_lxc_pol(
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
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_k_pc07_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part1_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part2_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigma_etc(rho, sigma, lapl, tau, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part3_v4rho4(rho, sigma, lapl, tau, v4rho4, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part4_v4rho3sigma(rho, sigma, lapl, tau, v4rho3sigma, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part5_v4rho3lapl_v4rho3tau(rho, sigma, lapl, tau, v4rho3lapl, v4rho3tau, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part6_v4rho2sigma2(rho, sigma, lapl, tau, v4rho2sigma2, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part7_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmalapl, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part8_v4rho2sigmalapl_17(rho, sigma, lapl, tau, v4rho2sigmalapl, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part9_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part10_v4rhosigma3(rho, sigma, lapl, tau, v4rhosigma3, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part11_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2lapl, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part12_v4rhosigma2lapl_23_v4rhosigma2tau(rho, sigma, lapl, tau, v4rhosigma2lapl, v4rhosigma2tau, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part13_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigmalapl2, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part14_v4rhosigmalapl2_17(rho, sigma, lapl, tau, v4rhosigmalapl2, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part15_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3_v4rholapl2tau_v4_etc(rho, sigma, lapl, tau, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part16_v4sigma4(rho, sigma, lapl, tau, v4sigma4, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part17_v4sigma3lapl_v4sigma3tau(rho, sigma, lapl, tau, v4sigma3lapl, v4sigma3tau, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part18_v4sigma2lapl2_v4sigma2lapltau_v4sigma2tau2(rho, sigma, lapl, tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part19_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4sigmatau3(rho, sigma, lapl, tau, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, param_a, param_b, dens_threshold, zeta_threshold);
    mgga_k_pc07_lxc_pol_part20_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4(rho, sigma, lapl, tau, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_a, param_b, dens_threshold, zeta_threshold);
}
