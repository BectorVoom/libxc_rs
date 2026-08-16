//! MGGA_C_M08 lxc pol kernel — lxc_pol (nested-by-output, 35 parts).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_m08_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_m08_lxc_pol_part1_v3rho3;
use part2::mgga_c_m08_lxc_pol_part2_v3rho2sigma_v3rho2lapl;
use part3::mgga_c_m08_lxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl;
use part4::mgga_c_m08_lxc_pol_part4_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc;
use part5::mgga_c_m08_lxc_pol_part5_v4rho4_0;
use part6::mgga_c_m08_lxc_pol_part6_v4rho4_1;
use part7::mgga_c_m08_lxc_pol_part7_v4rho4_2;
use part8::mgga_c_m08_lxc_pol_part8_v4rho4_3;
use part9::mgga_c_m08_lxc_pol_part9_v4rho4_4;
use part10::mgga_c_m08_lxc_pol_part10_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2;
use part11::mgga_c_m08_lxc_pol_part11_v4rho3sigma_3;
use part12::mgga_c_m08_lxc_pol_part12_v4rho3sigma_4;
use part13::mgga_c_m08_lxc_pol_part13_v4rho3sigma_5;
use part14::mgga_c_m08_lxc_pol_part14_v4rho3sigma_6;
use part15::mgga_c_m08_lxc_pol_part15_v4rho3sigma_7;
use part16::mgga_c_m08_lxc_pol_part16_v4rho3sigma_8;
use part17::mgga_c_m08_lxc_pol_part17_v4rho3sigma_9_v4rho3sigma_10_v4rho3sigma_11_v4rho3lapl;
use part18::mgga_c_m08_lxc_pol_part18_v4rho3tau_0;
use part19::mgga_c_m08_lxc_pol_part19_v4rho3tau_1;
use part20::mgga_c_m08_lxc_pol_part20_v4rho3tau_2;
use part21::mgga_c_m08_lxc_pol_part21_v4rho3tau_3;
use part22::mgga_c_m08_lxc_pol_part22_v4rho3tau_4;
use part23::mgga_c_m08_lxc_pol_part23_v4rho3tau_5;
use part24::mgga_c_m08_lxc_pol_part24_v4rho3tau_6;
use part25::mgga_c_m08_lxc_pol_part25_v4rho3tau_7;
use part26::mgga_c_m08_lxc_pol_part26_v4rho2sigma2_v4rho2sigmalapl;
use part27::mgga_c_m08_lxc_pol_part27_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau;
use part28::mgga_c_m08_lxc_pol_part28_v4rho2tau2_0_v4rho2tau2_1_v4rho2tau2_2;
use part29::mgga_c_m08_lxc_pol_part29_v4rho2tau2_3_v4rho2tau2_4;
use part30::mgga_c_m08_lxc_pol_part30_v4rho2tau2_5_v4rho2tau2_6;
use part31::mgga_c_m08_lxc_pol_part31_v4rho2tau2_7_v4rho2tau2_8;
use part32::mgga_c_m08_lxc_pol_part32_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhosigmalapl2_v_etc;
use part33::mgga_c_m08_lxc_pol_part33_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4_etc;
use part34::mgga_c_m08_lxc_pol_part34_v4tau4;

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
    mgga_c_m08_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part1_v3rho3(rho, sigma, lapl, tau, v3rho3, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part2_v3rho2sigma_v3rho2lapl(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, v3rhosigmalapl, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part4_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part5_v4rho4_0(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part6_v4rho4_1(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part7_v4rho4_2(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part8_v4rho4_3(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part9_v4rho4_4(rho, sigma, lapl, tau, v4rho4, param_m08_a_0, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part10_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part11_v4rho3sigma_3(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part12_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part13_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part14_v4rho3sigma_6(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part15_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part16_v4rho3sigma_8(rho, sigma, lapl, tau, v4rho3sigma, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part17_v4rho3sigma_9_v4rho3sigma_10_v4rho3sigma_11_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part18_v4rho3tau_0(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part19_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part20_v4rho3tau_2(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part21_v4rho3tau_3(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part22_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part23_v4rho3tau_5(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part24_v4rho3tau_6(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part25_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part26_v4rho2sigma2_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part27_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part28_v4rho2tau2_0_v4rho2tau2_1_v4rho2tau2_2(rho, sigma, lapl, tau, v4rho2tau2, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part29_v4rho2tau2_3_v4rho2tau2_4(rho, sigma, lapl, tau, v4rho2tau2, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part30_v4rho2tau2_5_v4rho2tau2_6(rho, sigma, lapl, tau, v4rho2tau2, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part31_v4rho2tau2_7_v4rho2tau2_8(rho, sigma, lapl, tau, v4rho2tau2, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part32_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhosigmalapl2_v_etc(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part33_v4rhotau3_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4_etc(rho, sigma, lapl, tau, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_0, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
    mgga_c_m08_lxc_pol_part34_v4tau4(rho, sigma, lapl, tau, v4tau4, param_m08_a_1, param_m08_a_2, param_m08_a_3, param_m08_a_4, param_m08_a_5, param_m08_a_6, param_m08_a_7, param_m08_a_8, param_m08_a_9, param_m08_a_10, param_m08_a_11, param_m08_b_1, param_m08_b_2, param_m08_b_3, param_m08_b_4, param_m08_b_5, param_m08_b_6, param_m08_b_7, param_m08_b_8, param_m08_b_9, param_m08_b_10, param_m08_b_11, dens_threshold, zeta_threshold);
}
