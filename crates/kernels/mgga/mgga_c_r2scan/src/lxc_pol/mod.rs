//! MGGA_C_R2SCAN lxc pol kernel — lxc_pol (nested-by-output, 53 parts).
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
mod part43;
mod part44;
mod part45;
mod part46;
mod part47;
mod part48;
mod part49;
mod part50;
mod part51;
mod part52;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

use part0::mgga_c_r2scan_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_r2scan_lxc_pol_part1_v3rho3;
use part2::mgga_c_r2scan_lxc_pol_part2_v3rho2sigma_v3rho2lapl;
use part3::mgga_c_r2scan_lxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc;
use part4::mgga_c_r2scan_lxc_pol_part4_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc;
use part5::mgga_c_r2scan_lxc_pol_part5_v4rho4_0;
use part6::mgga_c_r2scan_lxc_pol_part6_v4rho4_1;
use part7::mgga_c_r2scan_lxc_pol_part7_v4rho4_2;
use part8::mgga_c_r2scan_lxc_pol_part8_v4rho4_3;
use part9::mgga_c_r2scan_lxc_pol_part9_v4rho4_4;
use part10::mgga_c_r2scan_lxc_pol_part10_v4rho3sigma_0;
use part11::mgga_c_r2scan_lxc_pol_part11_v4rho3sigma_1;
use part12::mgga_c_r2scan_lxc_pol_part12_v4rho3sigma_2;
use part13::mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3;
use part14::mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4;
use part15::mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5;
use part16::mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6;
use part17::mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7;
use part18::mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8;
use part19::mgga_c_r2scan_lxc_pol_part19_v4rho3sigma_9;
use part20::mgga_c_r2scan_lxc_pol_part20_v4rho3sigma_10;
use part21::mgga_c_r2scan_lxc_pol_part21_v4rho3sigma_11_v4rho3lapl;
use part22::mgga_c_r2scan_lxc_pol_part22_v4rho3tau_0;
use part23::mgga_c_r2scan_lxc_pol_part23_v4rho3tau_1;
use part24::mgga_c_r2scan_lxc_pol_part24_v4rho3tau_2;
use part25::mgga_c_r2scan_lxc_pol_part25_v4rho3tau_3;
use part26::mgga_c_r2scan_lxc_pol_part26_v4rho3tau_4;
use part27::mgga_c_r2scan_lxc_pol_part27_v4rho3tau_5;
use part28::mgga_c_r2scan_lxc_pol_part28_v4rho3tau_6;
use part29::mgga_c_r2scan_lxc_pol_part29_v4rho3tau_7;
use part30::mgga_c_r2scan_lxc_pol_part30_v4rho2sigma2_0;
use part31::mgga_c_r2scan_lxc_pol_part31_v4rho2sigma2_1;
use part32::mgga_c_r2scan_lxc_pol_part32_v4rho2sigma2_2;
use part33::mgga_c_r2scan_lxc_pol_part33_v4rho2sigma2_3;
use part34::mgga_c_r2scan_lxc_pol_part34_v4rho2sigma2_4;
use part35::mgga_c_r2scan_lxc_pol_part35_v4rho2sigma2_5;
use part36::mgga_c_r2scan_lxc_pol_part36_v4rho2sigma2_6;
use part37::mgga_c_r2scan_lxc_pol_part37_v4rho2sigma2_7;
use part38::mgga_c_r2scan_lxc_pol_part38_v4rho2sigma2_8;
use part39::mgga_c_r2scan_lxc_pol_part39_v4rho2sigma2_9;
use part40::mgga_c_r2scan_lxc_pol_part40_v4rho2sigma2_10;
use part41::mgga_c_r2scan_lxc_pol_part41_v4rho2sigma2_11;
use part42::mgga_c_r2scan_lxc_pol_part42_v4rho2sigma2_12;
use part43::mgga_c_r2scan_lxc_pol_part43_v4rho2sigma2_13;
use part44::mgga_c_r2scan_lxc_pol_part44_v4rho2sigma2_14;
use part45::mgga_c_r2scan_lxc_pol_part45_v4rho2sigma2_15;
use part46::mgga_c_r2scan_lxc_pol_part46_v4rho2sigma2_16;
use part47::mgga_c_r2scan_lxc_pol_part47_v4rho2sigma2_17;
use part48::mgga_c_r2scan_lxc_pol_part48_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau;
use part49::mgga_c_r2scan_lxc_pol_part49_v4rho2tau2;
use part50::mgga_c_r2scan_lxc_pol_part50_v4rhosigma3_v4rhosigma2lapl;
use part51::mgga_c_r2scan_lxc_pol_part51_v4rhosigma2tau_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmat_etc;
use part52::mgga_c_r2scan_lxc_pol_part52_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4sigma2lapl_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol(
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
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_r2scan_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part1_v3rho3(rho, sigma, lapl, tau, v3rho3, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part2_v3rho2sigma_v3rho2lapl(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part3_v3rho2tau_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl_etc(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part4_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc(rho, sigma, lapl, tau, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part5_v4rho4_0(rho, sigma, lapl, tau, v4rho4, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part6_v4rho4_1(rho, sigma, lapl, tau, v4rho4, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part7_v4rho4_2(rho, sigma, lapl, tau, v4rho4, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part8_v4rho4_3(rho, sigma, lapl, tau, v4rho4, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part9_v4rho4_4(rho, sigma, lapl, tau, v4rho4, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part10_v4rho3sigma_0(rho, sigma, lapl, tau, v4rho3sigma, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part11_v4rho3sigma_1(rho, sigma, lapl, tau, v4rho3sigma, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part12_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part19_v4rho3sigma_9(rho, sigma, lapl, tau, v4rho3sigma, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part20_v4rho3sigma_10(rho, sigma, lapl, tau, v4rho3sigma, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part21_v4rho3sigma_11_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part22_v4rho3tau_0(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part23_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part24_v4rho3tau_2(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part25_v4rho3tau_3(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part26_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part27_v4rho3tau_5(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part28_v4rho3tau_6(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part29_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part30_v4rho2sigma2_0(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part31_v4rho2sigma2_1(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part32_v4rho2sigma2_2(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part33_v4rho2sigma2_3(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part34_v4rho2sigma2_4(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part35_v4rho2sigma2_5(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part36_v4rho2sigma2_6(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part37_v4rho2sigma2_7(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part38_v4rho2sigma2_8(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part39_v4rho2sigma2_9(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part40_v4rho2sigma2_10(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part41_v4rho2sigma2_11(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part42_v4rho2sigma2_12(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part43_v4rho2sigma2_13(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part44_v4rho2sigma2_14(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part45_v4rho2sigma2_15(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part46_v4rho2sigma2_16(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part47_v4rho2sigma2_17(rho, sigma, lapl, tau, v4rho2sigma2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part48_v4rho2sigmalapl_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau(rho, sigma, lapl, tau, v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part49_v4rho2tau2(rho, sigma, lapl, tau, v4rho2tau2, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part50_v4rhosigma3_v4rhosigma2lapl(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part51_v4rhosigma2tau_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmat_etc(rho, sigma, lapl, tau, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, param_eta, dens_threshold, zeta_threshold);
    mgga_c_r2scan_lxc_pol_part52_v4sigma4_v4sigma3lapl_v4sigma3tau_v4sigma2lapl2_v4sigma2lapl_etc(rho, sigma, lapl, tau, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_eta, dens_threshold, zeta_threshold);
}
