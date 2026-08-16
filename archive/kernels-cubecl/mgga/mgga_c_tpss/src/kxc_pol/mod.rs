//! MGGA_C_TPSS kxc pol kernel — kxc_pol (nested-by-output, 13 parts).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_tpss_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_tpss_kxc_pol_part1_v3rho3_0;
use part2::mgga_c_tpss_kxc_pol_part2_v3rho3_1;
use part3::mgga_c_tpss_kxc_pol_part3_v3rho3_2;
use part4::mgga_c_tpss_kxc_pol_part4_v3rho3_3;
use part5::mgga_c_tpss_kxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part6::mgga_c_tpss_kxc_pol_part6_v3rho2sigma_3;
use part7::mgga_c_tpss_kxc_pol_part7_v3rho2sigma_4;
use part8::mgga_c_tpss_kxc_pol_part8_v3rho2sigma_5;
use part9::mgga_c_tpss_kxc_pol_part9_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8_v3rho2lapl_0_v3rho_etc;
use part10::mgga_c_tpss_kxc_pol_part10_v3rho2tau;
use part11::mgga_c_tpss_kxc_pol_part11_v3rhosigma2_v3rhosigmalapl;
use part12::mgga_c_tpss_kxc_pol_part12_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_tpss_kxc_pol(
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
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_beta: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_tpss_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part1_v3rho3_0(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part2_v3rho3_1(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part3_v3rho3_2(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part4_v3rho3_3(rho, sigma, lapl, tau, v3rho3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part5_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part6_v3rho2sigma_3(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part7_v3rho2sigma_4(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part8_v3rho2sigma_5(rho, sigma, lapl, tau, v3rho2sigma, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part9_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8_v3rho2lapl_0_v3rho_etc(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part10_v3rho2tau(rho, sigma, lapl, tau, v3rho2tau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part11_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
    mgga_c_tpss_kxc_pol_part12_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_beta, param_d, dens_threshold, zeta_threshold);
}
