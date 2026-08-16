//! MGGA_X_MBRXC_BG lxc pol kernel — lxc_pol (nested-by-output, 28 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

use part0::mgga_x_mbrxc_bg_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_x_mbrxc_bg_lxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau;
use part2::mgga_x_mbrxc_bg_lxc_pol_part2_v3rhosigma2_v3rhosigmalapl;
use part3::mgga_x_mbrxc_bg_lxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau;
use part4::mgga_x_mbrxc_bg_lxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl;
use part5::mgga_x_mbrxc_bg_lxc_pol_part5_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc;
use part6::mgga_x_mbrxc_bg_lxc_pol_part6_v4rho4;
use part7::mgga_x_mbrxc_bg_lxc_pol_part7_v4rho3sigma_v4rho3lapl;
use part8::mgga_x_mbrxc_bg_lxc_pol_part8_v4rho3tau;
use part9::mgga_x_mbrxc_bg_lxc_pol_part9_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc;
use part10::mgga_x_mbrxc_bg_lxc_pol_part10_v4rho2sigma2_17_v4rho2sigmalapl;
use part11::mgga_x_mbrxc_bg_lxc_pol_part11_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc;
use part12::mgga_x_mbrxc_bg_lxc_pol_part12_v4rho2sigmatau_11_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2_etc;
use part13::mgga_x_mbrxc_bg_lxc_pol_part13_v4rho2sigmatau_17_v4rho2lapl2_v4rho2lapltau;
use part14::mgga_x_mbrxc_bg_lxc_pol_part14_v4rho2tau2_0_v4rho2tau2_1_v4rho2tau2_2_v4rho2tau2_3_v4rho2ta_etc;
use part15::mgga_x_mbrxc_bg_lxc_pol_part15_v4rho2tau2_8;
use part16::mgga_x_mbrxc_bg_lxc_pol_part16_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc;
use part17::mgga_x_mbrxc_bg_lxc_pol_part17_v4rhosigma3_19_v4rhosigma2lapl;
use part18::mgga_x_mbrxc_bg_lxc_pol_part18_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc;
use part19::mgga_x_mbrxc_bg_lxc_pol_part19_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc;
use part20::mgga_x_mbrxc_bg_lxc_pol_part20_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc;
use part21::mgga_x_mbrxc_bg_lxc_pol_part21_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11_v4rhosi_etc;
use part22::mgga_x_mbrxc_bg_lxc_pol_part22_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3;
use part23::mgga_x_mbrxc_bg_lxc_pol_part23_v4sigma4_v4sigma3lapl;
use part24::mgga_x_mbrxc_bg_lxc_pol_part24_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau;
use part25::mgga_x_mbrxc_bg_lxc_pol_part25_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2;
use part26::mgga_x_mbrxc_bg_lxc_pol_part26_v4sigmatau3_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3;
use part27::mgga_x_mbrxc_bg_lxc_pol_part27_v4tau4;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbrxc_bg_lxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_x_mbrxc_bg_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, v3rho3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part1_v3rho2sigma_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, v3rho2tau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part2_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part4_v3rhotau2_v3sigma3_v3sigma2lapl(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, v3sigma2lapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part5_v3sigma2tau_v3sigmalapl2_v3sigmalapltau_v3sigmatau2_v3lapl3__etc(rho, sigma, lapl, tau, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part6_v4rho4(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part7_v4rho3sigma_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part8_v4rho3tau(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part9_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part10_v4rho2sigma2_17_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part11_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part12_v4rho2sigmatau_11_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part13_v4rho2sigmatau_17_v4rho2lapl2_v4rho2lapltau(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part14_v4rho2tau2_0_v4rho2tau2_1_v4rho2tau2_2_v4rho2tau2_3_v4rho2ta_etc(rho, sigma, lapl, tau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part15_v4rho2tau2_8(rho, sigma, lapl, tau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part16_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part17_v4rhosigma3_19_v4rhosigma2lapl(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part18_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part19_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part20_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part21_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11_v4rhosi_etc(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part22_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3(rho, sigma, lapl, tau, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part23_v4sigma4_v4sigma3lapl(rho, sigma, lapl, tau, v4sigma4, v4sigma3lapl, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part24_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau(rho, sigma, lapl, tau, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part25_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2(rho, sigma, lapl, tau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part26_v4sigmatau3_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3(rho, sigma, lapl, tau, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, dens_threshold, zeta_threshold);
    mgga_x_mbrxc_bg_lxc_pol_part27_v4tau4(rho, sigma, lapl, tau, v4tau4, dens_threshold, zeta_threshold);
}
