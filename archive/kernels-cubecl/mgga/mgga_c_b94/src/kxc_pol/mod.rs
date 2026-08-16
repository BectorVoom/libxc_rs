//! MGGA_C_B94 kxc pol kernel — kxc_pol (nested-by-output, 43 parts).
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
mod part21;
mod part22;
mod part23;
mod part24;
mod part25;
mod part26;
mod part27;
mod part28;
mod part29;
mod part30;
mod part31;
mod part32;
mod part33;
mod part34;
mod part35;
mod part36;
mod part37;
mod part38;
mod part39;
mod part40;
mod part41;
mod part42;

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

use part0::mgga_c_b94_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::mgga_c_b94_kxc_pol_part1_v2rhosigma_v2rholapl;
use part2::mgga_c_b94_kxc_pol_part2_v2rhotau_v2sigma2_v2sigmalapl;
use part3::mgga_c_b94_kxc_pol_part3_v2sigmatau_v2lapl2_v2lapltau_v2tau2;
use part4::mgga_c_b94_kxc_pol_part4_v3rho3_0;
use part5::mgga_c_b94_kxc_pol_part5_v3rho3_1;
use part6::mgga_c_b94_kxc_pol_part6_v3rho3_2;
use part7::mgga_c_b94_kxc_pol_part7_v3rho3_3;
use part8::mgga_c_b94_kxc_pol_part8_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part9::mgga_c_b94_kxc_pol_part9_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5;
use part10::mgga_c_b94_kxc_pol_part10_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8;
use part11::mgga_c_b94_kxc_pol_part11_v3rho2lapl_0_v3rho2lapl_1;
use part12::mgga_c_b94_kxc_pol_part12_v3rho2lapl_2_v3rho2lapl_3_v3rho2lapl_4;
use part13::mgga_c_b94_kxc_pol_part13_v3rho2lapl_5;
use part14::mgga_c_b94_kxc_pol_part14_v3rho2tau_0_v3rho2tau_1;
use part15::mgga_c_b94_kxc_pol_part15_v3rho2tau_2_v3rho2tau_3;
use part16::mgga_c_b94_kxc_pol_part16_v3rho2tau_4_v3rho2tau_5;
use part17::mgga_c_b94_kxc_pol_part17_v3rhosigma2_0_v3rhosigma2_1_v3rhosigma2_2_v3rhosigma2_3_v3rh_etc;
use part18::mgga_c_b94_kxc_pol_part18_v3rhosigma2_6_v3rhosigma2_7_v3rhosigma2_8_v3rhosigma2_9_v3rh_etc;
use part19::mgga_c_b94_kxc_pol_part19_v3rhosigmalapl_0_v3rhosigmalapl_1_v3rhosigmalapl_2_v3rhosigm_etc;
use part20::mgga_c_b94_kxc_pol_part20_v3rhosigmalapl_5_v3rhosigmalapl_6_v3rhosigmalapl_7_v3rhosigm_etc;
use part21::mgga_c_b94_kxc_pol_part21_v3rhosigmalapl_11;
use part22::mgga_c_b94_kxc_pol_part22_v3rhosigmatau_0_v3rhosigmatau_1_v3rhosigmatau_2_v3rhosigmata_etc;
use part23::mgga_c_b94_kxc_pol_part23_v3rhosigmatau_5_v3rhosigmatau_6_v3rhosigmatau_7_v3rhosigmata_etc;
use part24::mgga_c_b94_kxc_pol_part24_v3rhosigmatau_11;
use part25::mgga_c_b94_kxc_pol_part25_v3rholapl2_0_v3rholapl2_1_v3rholapl2_2_v3rholapl2_3_v3rholap_etc;
use part26::mgga_c_b94_kxc_pol_part26_v3rholapl2_5;
use part27::mgga_c_b94_kxc_pol_part27_v3rholapltau_0_v3rholapltau_1_v3rholapltau_2_v3rholapltau_3;
use part28::mgga_c_b94_kxc_pol_part28_v3rholapltau_4_v3rholapltau_5_v3rholapltau_6;
use part29::mgga_c_b94_kxc_pol_part29_v3rholapltau_7;
use part30::mgga_c_b94_kxc_pol_part30_v3rhotau2_0_v3rhotau2_1_v3rhotau2_2;
use part31::mgga_c_b94_kxc_pol_part31_v3rhotau2_3_v3rhotau2_4_v3rhotau2_5;
use part32::mgga_c_b94_kxc_pol_part32_v3sigma3;
use part33::mgga_c_b94_kxc_pol_part33_v3sigma2lapl;
use part34::mgga_c_b94_kxc_pol_part34_v3sigma2tau;
use part35::mgga_c_b94_kxc_pol_part35_v3sigmalapl2;
use part36::mgga_c_b94_kxc_pol_part36_v3sigmalapltau_0_v3sigmalapltau_1_v3sigmalapltau_2_v3sigmala_etc;
use part37::mgga_c_b94_kxc_pol_part37_v3sigmalapltau_11;
use part38::mgga_c_b94_kxc_pol_part38_v3sigmatau2;
use part39::mgga_c_b94_kxc_pol_part39_v3lapl3;
use part40::mgga_c_b94_kxc_pol_part40_v3lapl2tau;
use part41::mgga_c_b94_kxc_pol_part41_v3lapltau2;
use part42::mgga_c_b94_kxc_pol_part42_v3tau3;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_kxc_pol(
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
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_b94_kxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part1_v2rhosigma_v2rholapl(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part2_v2rhotau_v2sigma2_v2sigmalapl(rho, sigma, lapl, tau, v2rhotau, v2sigma2, v2sigmalapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part3_v2sigmatau_v2lapl2_v2lapltau_v2tau2(rho, sigma, lapl, tau, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part4_v3rho3_0(rho, sigma, lapl, tau, v3rho3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part5_v3rho3_1(rho, sigma, lapl, tau, v3rho3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part6_v3rho3_2(rho, sigma, lapl, tau, v3rho3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part7_v3rho3_3(rho, sigma, lapl, tau, v3rho3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part8_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, lapl, tau, v3rho2sigma, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part9_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5(rho, sigma, lapl, tau, v3rho2sigma, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part10_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8(rho, sigma, lapl, tau, v3rho2sigma, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part11_v3rho2lapl_0_v3rho2lapl_1(rho, sigma, lapl, tau, v3rho2lapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part12_v3rho2lapl_2_v3rho2lapl_3_v3rho2lapl_4(rho, sigma, lapl, tau, v3rho2lapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part13_v3rho2lapl_5(rho, sigma, lapl, tau, v3rho2lapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part14_v3rho2tau_0_v3rho2tau_1(rho, sigma, lapl, tau, v3rho2tau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part15_v3rho2tau_2_v3rho2tau_3(rho, sigma, lapl, tau, v3rho2tau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part16_v3rho2tau_4_v3rho2tau_5(rho, sigma, lapl, tau, v3rho2tau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part17_v3rhosigma2_0_v3rhosigma2_1_v3rhosigma2_2_v3rhosigma2_3_v3rh_etc(rho, sigma, lapl, tau, v3rhosigma2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part18_v3rhosigma2_6_v3rhosigma2_7_v3rhosigma2_8_v3rhosigma2_9_v3rh_etc(rho, sigma, lapl, tau, v3rhosigma2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part19_v3rhosigmalapl_0_v3rhosigmalapl_1_v3rhosigmalapl_2_v3rhosigm_etc(rho, sigma, lapl, tau, v3rhosigmalapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part20_v3rhosigmalapl_5_v3rhosigmalapl_6_v3rhosigmalapl_7_v3rhosigm_etc(rho, sigma, lapl, tau, v3rhosigmalapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part21_v3rhosigmalapl_11(rho, sigma, lapl, tau, v3rhosigmalapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part22_v3rhosigmatau_0_v3rhosigmatau_1_v3rhosigmatau_2_v3rhosigmata_etc(rho, sigma, lapl, tau, v3rhosigmatau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part23_v3rhosigmatau_5_v3rhosigmatau_6_v3rhosigmatau_7_v3rhosigmata_etc(rho, sigma, lapl, tau, v3rhosigmatau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part24_v3rhosigmatau_11(rho, sigma, lapl, tau, v3rhosigmatau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part25_v3rholapl2_0_v3rholapl2_1_v3rholapl2_2_v3rholapl2_3_v3rholap_etc(rho, sigma, lapl, tau, v3rholapl2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part26_v3rholapl2_5(rho, sigma, lapl, tau, v3rholapl2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part27_v3rholapltau_0_v3rholapltau_1_v3rholapltau_2_v3rholapltau_3(rho, sigma, lapl, tau, v3rholapltau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part28_v3rholapltau_4_v3rholapltau_5_v3rholapltau_6(rho, sigma, lapl, tau, v3rholapltau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part29_v3rholapltau_7(rho, sigma, lapl, tau, v3rholapltau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part30_v3rhotau2_0_v3rhotau2_1_v3rhotau2_2(rho, sigma, lapl, tau, v3rhotau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part31_v3rhotau2_3_v3rhotau2_4_v3rhotau2_5(rho, sigma, lapl, tau, v3rhotau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part32_v3sigma3(rho, sigma, lapl, tau, v3sigma3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part33_v3sigma2lapl(rho, sigma, lapl, tau, v3sigma2lapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part34_v3sigma2tau(rho, sigma, lapl, tau, v3sigma2tau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part35_v3sigmalapl2(rho, sigma, lapl, tau, v3sigmalapl2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part36_v3sigmalapltau_0_v3sigmalapltau_1_v3sigmalapltau_2_v3sigmala_etc(rho, sigma, lapl, tau, v3sigmalapltau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part37_v3sigmalapltau_11(rho, sigma, lapl, tau, v3sigmalapltau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part38_v3sigmatau2(rho, sigma, lapl, tau, v3sigmatau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part39_v3lapl3(rho, sigma, lapl, tau, v3lapl3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part40_v3lapl2tau(rho, sigma, lapl, tau, v3lapl2tau, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part41_v3lapltau2(rho, sigma, lapl, tau, v3lapltau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_kxc_pol_part42_v3tau3(rho, sigma, lapl, tau, v3tau3, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
}
