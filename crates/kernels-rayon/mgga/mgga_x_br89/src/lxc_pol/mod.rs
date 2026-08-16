//! MGGA_X_BR89 lxc pol kernel — lxc_pol (nested-by-output, 47 parts).
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

use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::mgga_x_br89_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_br89_lxc_pol_part1_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc;
use part2::mgga_x_br89_lxc_pol_part2_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
use part3::mgga_x_br89_lxc_pol_part3_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3;
use part4::mgga_x_br89_lxc_pol_part4_v4rho4;
use part5::mgga_x_br89_lxc_pol_part5_v4rho3sigma;
use part6::mgga_x_br89_lxc_pol_part6_v4rho3lapl;
use part7::mgga_x_br89_lxc_pol_part7_v4rho3tau;
use part8::mgga_x_br89_lxc_pol_part8_v4rho2sigma2;
use part9::mgga_x_br89_lxc_pol_part9_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc;
use part10::mgga_x_br89_lxc_pol_part10_v4rho2sigmalapl_17;
use part11::mgga_x_br89_lxc_pol_part11_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc;
use part12::mgga_x_br89_lxc_pol_part12_v4rho2sigmatau_17;
use part13::mgga_x_br89_lxc_pol_part13_v4rho2lapl2;
use part14::mgga_x_br89_lxc_pol_part14_v4rho2lapltau_0_v4rho2lapltau_1_v4rho2lapltau_2_v4rho2laplta_etc;
use part15::mgga_x_br89_lxc_pol_part15_v4rho2lapltau_11;
use part16::mgga_x_br89_lxc_pol_part16_v4rho2tau2;
use part17::mgga_x_br89_lxc_pol_part17_v4rhosigma3;
use part18::mgga_x_br89_lxc_pol_part18_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc;
use part19::mgga_x_br89_lxc_pol_part19_v4rhosigma2lapl_23;
use part20::mgga_x_br89_lxc_pol_part20_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc;
use part21::mgga_x_br89_lxc_pol_part21_v4rhosigma2tau_23;
use part22::mgga_x_br89_lxc_pol_part22_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc;
use part23::mgga_x_br89_lxc_pol_part23_v4rhosigmalapl2_17;
use part24::mgga_x_br89_lxc_pol_part24_v4rhosigmalapltau_0_v4rhosigmalapltau_1_v4rhosigmalapltau_2__etc;
use part25::mgga_x_br89_lxc_pol_part25_v4rhosigmalapltau_12_v4rhosigmalapltau_13_v4rhosigmalapltau__etc;
use part26::mgga_x_br89_lxc_pol_part26_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc;
use part27::mgga_x_br89_lxc_pol_part27_v4rhosigmatau2_17;
use part28::mgga_x_br89_lxc_pol_part28_v4rholapl3;
use part29::mgga_x_br89_lxc_pol_part29_v4rholapl2tau_0_v4rholapl2tau_1_v4rholapl2tau_2_v4rholapl2ta_etc;
use part30::mgga_x_br89_lxc_pol_part30_v4rholapl2tau_11;
use part31::mgga_x_br89_lxc_pol_part31_v4rholapltau2_0_v4rholapltau2_1_v4rholapltau2_2_v4rholapltau_etc;
use part32::mgga_x_br89_lxc_pol_part32_v4rholapltau2_11;
use part33::mgga_x_br89_lxc_pol_part33_v4rhotau3;
use part34::mgga_x_br89_lxc_pol_part34_v4sigma4_v4sigma3lapl;
use part35::mgga_x_br89_lxc_pol_part35_v4sigma3tau;
use part36::mgga_x_br89_lxc_pol_part36_v4sigma2lapl2;
use part37::mgga_x_br89_lxc_pol_part37_v4sigma2lapltau;
use part38::mgga_x_br89_lxc_pol_part38_v4sigma2tau2;
use part39::mgga_x_br89_lxc_pol_part39_v4sigmalapl3;
use part40::mgga_x_br89_lxc_pol_part40_v4sigmalapl2tau;
use part41::mgga_x_br89_lxc_pol_part41_v4sigmalapltau2;
use part42::mgga_x_br89_lxc_pol_part42_v4sigmatau3;
use part43::mgga_x_br89_lxc_pol_part43_v4lapl4_v4lapl3tau;
use part44::mgga_x_br89_lxc_pol_part44_v4lapl2tau2;
use part45::mgga_x_br89_lxc_pol_part45_v4lapltau3;
use part46::mgga_x_br89_lxc_pol_part46_v4tau4;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_lxc_pol(
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
    param_at: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_br89_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part1_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part2_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part3_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3(rho, sigma, lapl, tau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part4_v4rho4(rho, sigma, lapl, tau, v4rho4, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part5_v4rho3sigma(rho, sigma, lapl, tau, v4rho3sigma, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part6_v4rho3lapl(rho, sigma, lapl, tau, v4rho3lapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part7_v4rho3tau(rho, sigma, lapl, tau, v4rho3tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part8_v4rho2sigma2(rho, sigma, lapl, tau, v4rho2sigma2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part9_v4rho2sigmalapl_0_v4rho2sigmalapl_1_v4rho2sigmalapl_2_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmalapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part10_v4rho2sigmalapl_17(rho, sigma, lapl, tau, v4rho2sigmalapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part11_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part12_v4rho2sigmatau_17(rho, sigma, lapl, tau, v4rho2sigmatau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part13_v4rho2lapl2(rho, sigma, lapl, tau, v4rho2lapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part14_v4rho2lapltau_0_v4rho2lapltau_1_v4rho2lapltau_2_v4rho2laplta_etc(rho, sigma, lapl, tau, v4rho2lapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part15_v4rho2lapltau_11(rho, sigma, lapl, tau, v4rho2lapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part16_v4rho2tau2(rho, sigma, lapl, tau, v4rho2tau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part17_v4rhosigma3(rho, sigma, lapl, tau, v4rhosigma3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part18_v4rhosigma2lapl_0_v4rhosigma2lapl_1_v4rhosigma2lapl_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2lapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part19_v4rhosigma2lapl_23(rho, sigma, lapl, tau, v4rhosigma2lapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part20_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part21_v4rhosigma2tau_23(rho, sigma, lapl, tau, v4rhosigma2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part22_v4rhosigmalapl2_0_v4rhosigmalapl2_1_v4rhosigmalapl2_2_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigmalapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part23_v4rhosigmalapl2_17(rho, sigma, lapl, tau, v4rhosigmalapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part24_v4rhosigmalapltau_0_v4rhosigmalapltau_1_v4rhosigmalapltau_2__etc(rho, sigma, lapl, tau, v4rhosigmalapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part25_v4rhosigmalapltau_12_v4rhosigmalapltau_13_v4rhosigmalapltau__etc(rho, sigma, lapl, tau, v4rhosigmalapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part26_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigmatau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part27_v4rhosigmatau2_17(rho, sigma, lapl, tau, v4rhosigmatau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part28_v4rholapl3(rho, sigma, lapl, tau, v4rholapl3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part29_v4rholapl2tau_0_v4rholapl2tau_1_v4rholapl2tau_2_v4rholapl2ta_etc(rho, sigma, lapl, tau, v4rholapl2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part30_v4rholapl2tau_11(rho, sigma, lapl, tau, v4rholapl2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part31_v4rholapltau2_0_v4rholapltau2_1_v4rholapltau2_2_v4rholapltau_etc(rho, sigma, lapl, tau, v4rholapltau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part32_v4rholapltau2_11(rho, sigma, lapl, tau, v4rholapltau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part33_v4rhotau3(rho, sigma, lapl, tau, v4rhotau3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part34_v4sigma4_v4sigma3lapl(rho, sigma, lapl, tau, v4sigma4, v4sigma3lapl, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part35_v4sigma3tau(rho, sigma, lapl, tau, v4sigma3tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part36_v4sigma2lapl2(rho, sigma, lapl, tau, v4sigma2lapl2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part37_v4sigma2lapltau(rho, sigma, lapl, tau, v4sigma2lapltau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part38_v4sigma2tau2(rho, sigma, lapl, tau, v4sigma2tau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part39_v4sigmalapl3(rho, sigma, lapl, tau, v4sigmalapl3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part40_v4sigmalapl2tau(rho, sigma, lapl, tau, v4sigmalapl2tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part41_v4sigmalapltau2(rho, sigma, lapl, tau, v4sigmalapltau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part42_v4sigmatau3(rho, sigma, lapl, tau, v4sigmatau3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part43_v4lapl4_v4lapl3tau(rho, sigma, lapl, tau, v4lapl4, v4lapl3tau, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part44_v4lapl2tau2(rho, sigma, lapl, tau, v4lapl2tau2, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part45_v4lapltau3(rho, sigma, lapl, tau, v4lapltau3, param_at, param_gamma, dens_threshold, zeta_threshold);
    mgga_x_br89_lxc_pol_part46_v4tau4(rho, sigma, lapl, tau, v4tau4, param_at, param_gamma, dens_threshold, zeta_threshold);
}
