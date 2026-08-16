//! MGGA_X_BR89_EXPLICIT lxc pol kernel — lxc_pol (nested-by-output, 65 parts).
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
mod part53;
mod part54;
mod part55;
mod part56;
mod part57;
mod part58;
mod part59;
mod part60;
mod part61;
mod part62;
mod part63;
mod part64;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

use part0::mgga_x_br89_explicit_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_br89_explicit_lxc_pol_part1_v3rho3_v3rho2sigma;
use part2::mgga_x_br89_explicit_lxc_pol_part2_v3rho2lapl;
use part3::mgga_x_br89_explicit_lxc_pol_part3_v3rho2tau_v3rhosigma2;
use part4::mgga_x_br89_explicit_lxc_pol_part4_v3rhosigmalapl;
use part5::mgga_x_br89_explicit_lxc_pol_part5_v3rhosigmatau;
use part6::mgga_x_br89_explicit_lxc_pol_part6_v3rholapl2;
use part7::mgga_x_br89_explicit_lxc_pol_part7_v3rholapltau;
use part8::mgga_x_br89_explicit_lxc_pol_part8_v3rhotau2_v3sigma3;
use part9::mgga_x_br89_explicit_lxc_pol_part9_v3sigma2lapl_v3sigma2tau;
use part10::mgga_x_br89_explicit_lxc_pol_part10_v3sigmalapl2_v3sigmalapltau;
use part11::mgga_x_br89_explicit_lxc_pol_part11_v3sigmatau2_v3lapl3;
use part12::mgga_x_br89_explicit_lxc_pol_part12_v3lapl2tau_v3lapltau2_v3tau3;
use part13::mgga_x_br89_explicit_lxc_pol_part13_v4rho4;
use part14::mgga_x_br89_explicit_lxc_pol_part14_v4rho3sigma;
use part15::mgga_x_br89_explicit_lxc_pol_part15_v4rho3lapl;
use part16::mgga_x_br89_explicit_lxc_pol_part16_v4rho3tau;
use part17::mgga_x_br89_explicit_lxc_pol_part17_v4rho2sigma2;
use part18::mgga_x_br89_explicit_lxc_pol_part18_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc;
use part19::mgga_x_br89_explicit_lxc_pol_part19_v4rho2sigmalapl_6_v4rho2sigmalapl_7_v4rho2sigmalapl_8_v4rho2_etc;
use part20::mgga_x_br89_explicit_lxc_pol_part20_v4rho2sigmalapl_17;
use part21::mgga_x_br89_explicit_lxc_pol_part21_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc;
use part22::mgga_x_br89_explicit_lxc_pol_part22_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc;
use part23::mgga_x_br89_explicit_lxc_pol_part23_v4rho2sigmatau_17;
use part24::mgga_x_br89_explicit_lxc_pol_part24_v4rho2lapl2;
use part25::mgga_x_br89_explicit_lxc_pol_part25_v4rho2lapltau_0_v4rho2lapltau_1_v4rho2lapltau_2_v4rho2laplta_etc;
use part26::mgga_x_br89_explicit_lxc_pol_part26_v4rho2lapltau_4_v4rho2lapltau_5_v4rho2lapltau_6_v4rho2laplta_etc;
use part27::mgga_x_br89_explicit_lxc_pol_part27_v4rho2lapltau_11;
use part28::mgga_x_br89_explicit_lxc_pol_part28_v4rho2tau2;
use part29::mgga_x_br89_explicit_lxc_pol_part29_v4rhosigma3;
use part30::mgga_x_br89_explicit_lxc_pol_part30_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc;
use part31::mgga_x_br89_explicit_lxc_pol_part31_v4rhosigma2lapl_12_v4rhosigma2lapl_13_v4rhosigma2lapl_14_v4r_etc;
use part32::mgga_x_br89_explicit_lxc_pol_part32_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc;
use part33::mgga_x_br89_explicit_lxc_pol_part33_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc;
use part34::mgga_x_br89_explicit_lxc_pol_part34_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc;
use part35::mgga_x_br89_explicit_lxc_pol_part35_v4rhosigmalapl2_9_v4rhosigmalapl2_10_v4rhosigmalapl2_11_v4rh_etc;
use part36::mgga_x_br89_explicit_lxc_pol_part36_v4rhosigmalapltau_0_v4rhosigmalapltau_1_v4rhosigmalapltau_2__etc;
use part37::mgga_x_br89_explicit_lxc_pol_part37_v4rhosigmalapltau_11_v4rhosigmalapltau_12_v4rhosigmalapltau__etc;
use part38::mgga_x_br89_explicit_lxc_pol_part38_v4rhosigmalapltau_23;
use part39::mgga_x_br89_explicit_lxc_pol_part39_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc;
use part40::mgga_x_br89_explicit_lxc_pol_part40_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11_v4rhosi_etc;
use part41::mgga_x_br89_explicit_lxc_pol_part41_v4rholapl3;
use part42::mgga_x_br89_explicit_lxc_pol_part42_v4rholapl2tau_0_v4rholapl2tau_1_v4rholapl2tau_2_v4rholapl2ta_etc;
use part43::mgga_x_br89_explicit_lxc_pol_part43_v4rholapl2tau_6_v4rholapl2tau_7_v4rholapl2tau_8_v4rholapl2ta_etc;
use part44::mgga_x_br89_explicit_lxc_pol_part44_v4rholapltau2_0_v4rholapltau2_1_v4rholapltau2_2_v4rholapltau_etc;
use part45::mgga_x_br89_explicit_lxc_pol_part45_v4rholapltau2_6_v4rholapltau2_7_v4rholapltau2_8_v4rholapltau_etc;
use part46::mgga_x_br89_explicit_lxc_pol_part46_v4rhotau3;
use part47::mgga_x_br89_explicit_lxc_pol_part47_v4sigma4;
use part48::mgga_x_br89_explicit_lxc_pol_part48_v4sigma3lapl;
use part49::mgga_x_br89_explicit_lxc_pol_part49_v4sigma3tau;
use part50::mgga_x_br89_explicit_lxc_pol_part50_v4sigma2lapl2;
use part51::mgga_x_br89_explicit_lxc_pol_part51_v4sigma2lapltau_0_v4sigma2lapltau_1_v4sigma2lapltau_2_v4sigm_etc;
use part52::mgga_x_br89_explicit_lxc_pol_part52_v4sigma2lapltau_23;
use part53::mgga_x_br89_explicit_lxc_pol_part53_v4sigma2tau2;
use part54::mgga_x_br89_explicit_lxc_pol_part54_v4sigmalapl3;
use part55::mgga_x_br89_explicit_lxc_pol_part55_v4sigmalapl2tau_0_v4sigmalapl2tau_1_v4sigmalapl2tau_2_v4sigm_etc;
use part56::mgga_x_br89_explicit_lxc_pol_part56_v4sigmalapl2tau_17;
use part57::mgga_x_br89_explicit_lxc_pol_part57_v4sigmalapltau2_0_v4sigmalapltau2_1_v4sigmalapltau2_2_v4sigm_etc;
use part58::mgga_x_br89_explicit_lxc_pol_part58_v4sigmalapltau2_17;
use part59::mgga_x_br89_explicit_lxc_pol_part59_v4sigmatau3;
use part60::mgga_x_br89_explicit_lxc_pol_part60_v4lapl4;
use part61::mgga_x_br89_explicit_lxc_pol_part61_v4lapl3tau;
use part62::mgga_x_br89_explicit_lxc_pol_part62_v4lapl2tau2;
use part63::mgga_x_br89_explicit_lxc_pol_part63_v4lapltau3;
use part64::mgga_x_br89_explicit_lxc_pol_part64_v4tau4;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_explicit_lxc_pol(
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
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_br89_explicit_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part1_v3rho3_v3rho2sigma(rho, sigma, lapl, tau, v3rho3, v3rho2sigma, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part2_v3rho2lapl(rho, sigma, lapl, tau, v3rho2lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part3_v3rho2tau_v3rhosigma2(rho, sigma, lapl, tau, v3rho2tau, v3rhosigma2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part4_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part5_v3rhosigmatau(rho, sigma, lapl, tau, v3rhosigmatau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part6_v3rholapl2(rho, sigma, lapl, tau, v3rholapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part7_v3rholapltau(rho, sigma, lapl, tau, v3rholapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part8_v3rhotau2_v3sigma3(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part9_v3sigma2lapl_v3sigma2tau(rho, sigma, lapl, tau, v3sigma2lapl, v3sigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part10_v3sigmalapl2_v3sigmalapltau(rho, sigma, lapl, tau, v3sigmalapl2, v3sigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part11_v3sigmatau2_v3lapl3(rho, sigma, lapl, tau, v3sigmatau2, v3lapl3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part12_v3lapl2tau_v3lapltau2_v3tau3(rho, sigma, lapl, tau, v3lapl2tau, v3lapltau2, v3tau3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part13_v4rho4(rho, sigma, lapl, tau, v4rho4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part14_v4rho3sigma(rho, sigma, lapl, tau, v4rho3sigma, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part15_v4rho3lapl(rho, sigma, lapl, tau, v4rho3lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part16_v4rho3tau(rho, sigma, lapl, tau, v4rho3tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part17_v4rho2sigma2(rho, sigma, lapl, tau, v4rho2sigma2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part18_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part19_v4rho2sigmalapl_6_v4rho2sigmalapl_7_v4rho2sigmalapl_8_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part20_v4rho2sigmalapl_17(rho, sigma, lapl, tau, v4rho2sigmalapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part21_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part22_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part23_v4rho2sigmatau_17(rho, sigma, lapl, tau, v4rho2sigmatau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part24_v4rho2lapl2(rho, sigma, lapl, tau, v4rho2lapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part25_v4rho2lapltau_0_v4rho2lapltau_1_v4rho2lapltau_2_v4rho2laplta_etc(rho, sigma, lapl, tau, v4rho2lapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part26_v4rho2lapltau_4_v4rho2lapltau_5_v4rho2lapltau_6_v4rho2laplta_etc(rho, sigma, lapl, tau, v4rho2lapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part27_v4rho2lapltau_11(rho, sigma, lapl, tau, v4rho2lapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part28_v4rho2tau2(rho, sigma, lapl, tau, v4rho2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part29_v4rhosigma3(rho, sigma, lapl, tau, v4rhosigma3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part30_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part31_v4rhosigma2lapl_12_v4rhosigma2lapl_13_v4rhosigma2lapl_14_v4r_etc(rho, sigma, lapl, tau, v4rhosigma2lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part32_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part33_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part34_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigmalapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part35_v4rhosigmalapl2_9_v4rhosigmalapl2_10_v4rhosigmalapl2_11_v4rh_etc(rho, sigma, lapl, tau, v4rhosigmalapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part36_v4rhosigmalapltau_0_v4rhosigmalapltau_1_v4rhosigmalapltau_2__etc(rho, sigma, lapl, tau, v4rhosigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part37_v4rhosigmalapltau_11_v4rhosigmalapltau_12_v4rhosigmalapltau__etc(rho, sigma, lapl, tau, v4rhosigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part38_v4rhosigmalapltau_23(rho, sigma, lapl, tau, v4rhosigmalapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part39_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigmatau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part40_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11_v4rhosi_etc(rho, sigma, lapl, tau, v4rhosigmatau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part41_v4rholapl3(rho, sigma, lapl, tau, v4rholapl3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part42_v4rholapl2tau_0_v4rholapl2tau_1_v4rholapl2tau_2_v4rholapl2ta_etc(rho, sigma, lapl, tau, v4rholapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part43_v4rholapl2tau_6_v4rholapl2tau_7_v4rholapl2tau_8_v4rholapl2ta_etc(rho, sigma, lapl, tau, v4rholapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part44_v4rholapltau2_0_v4rholapltau2_1_v4rholapltau2_2_v4rholapltau_etc(rho, sigma, lapl, tau, v4rholapltau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part45_v4rholapltau2_6_v4rholapltau2_7_v4rholapltau2_8_v4rholapltau_etc(rho, sigma, lapl, tau, v4rholapltau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part46_v4rhotau3(rho, sigma, lapl, tau, v4rhotau3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part47_v4sigma4(rho, sigma, lapl, tau, v4sigma4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part48_v4sigma3lapl(rho, sigma, lapl, tau, v4sigma3lapl, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part49_v4sigma3tau(rho, sigma, lapl, tau, v4sigma3tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part50_v4sigma2lapl2(rho, sigma, lapl, tau, v4sigma2lapl2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part51_v4sigma2lapltau_0_v4sigma2lapltau_1_v4sigma2lapltau_2_v4sigm_etc(rho, sigma, lapl, tau, v4sigma2lapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part52_v4sigma2lapltau_23(rho, sigma, lapl, tau, v4sigma2lapltau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part53_v4sigma2tau2(rho, sigma, lapl, tau, v4sigma2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part54_v4sigmalapl3(rho, sigma, lapl, tau, v4sigmalapl3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part55_v4sigmalapl2tau_0_v4sigmalapl2tau_1_v4sigmalapl2tau_2_v4sigm_etc(rho, sigma, lapl, tau, v4sigmalapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part56_v4sigmalapl2tau_17(rho, sigma, lapl, tau, v4sigmalapl2tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part57_v4sigmalapltau2_0_v4sigmalapltau2_1_v4sigmalapltau2_2_v4sigm_etc(rho, sigma, lapl, tau, v4sigmalapltau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part58_v4sigmalapltau2_17(rho, sigma, lapl, tau, v4sigmalapltau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part59_v4sigmatau3(rho, sigma, lapl, tau, v4sigmatau3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part60_v4lapl4(rho, sigma, lapl, tau, v4lapl4, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part61_v4lapl3tau(rho, sigma, lapl, tau, v4lapl3tau, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part62_v4lapl2tau2(rho, sigma, lapl, tau, v4lapl2tau2, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part63_v4lapltau3(rho, sigma, lapl, tau, v4lapltau3, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_explicit_lxc_pol_part64_v4tau4(rho, sigma, lapl, tau, v4tau4, param_gamma, dens_threshold, zeta_threshold);
}
