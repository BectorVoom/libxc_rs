//! HYB_MGGA_X_JS18 lxc pol kernel — lxc_pol (nested-by-output, 38 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

use part0::hyb_mgga_x_js18_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::hyb_mgga_x_js18_lxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;
use part2::hyb_mgga_x_js18_lxc_pol_part2_v3rho3;
use part3::hyb_mgga_x_js18_lxc_pol_part3_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2_v3rho2sigma_3_v3rh_etc;
use part4::hyb_mgga_x_js18_lxc_pol_part4_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8;
use part5::hyb_mgga_x_js18_lxc_pol_part5_v3rho2lapl_v3rho2tau;
use part6::hyb_mgga_x_js18_lxc_pol_part6_v3rhosigma2_v3rhosigmalapl;
use part7::hyb_mgga_x_js18_lxc_pol_part7_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2;
use part8::hyb_mgga_x_js18_lxc_pol_part8_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc;
use part9::hyb_mgga_x_js18_lxc_pol_part9_v4rho4_0;
use part10::hyb_mgga_x_js18_lxc_pol_part10_v4rho4_1;
use part11::hyb_mgga_x_js18_lxc_pol_part11_v4rho4_2;
use part12::hyb_mgga_x_js18_lxc_pol_part12_v4rho4_3;
use part13::hyb_mgga_x_js18_lxc_pol_part13_v4rho4_4;
use part14::hyb_mgga_x_js18_lxc_pol_part14_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2;
use part15::hyb_mgga_x_js18_lxc_pol_part15_v4rho3sigma_3_v4rho3sigma_4;
use part16::hyb_mgga_x_js18_lxc_pol_part16_v4rho3sigma_5;
use part17::hyb_mgga_x_js18_lxc_pol_part17_v4rho3sigma_6_v4rho3sigma_7;
use part18::hyb_mgga_x_js18_lxc_pol_part18_v4rho3sigma_8_v4rho3sigma_9_v4rho3sigma_10;
use part19::hyb_mgga_x_js18_lxc_pol_part19_v4rho3sigma_11_v4rho3lapl_v4rho3tau_0_v4rho3tau_1;
use part20::hyb_mgga_x_js18_lxc_pol_part20_v4rho3tau_2_v4rho3tau_3_v4rho3tau_4;
use part21::hyb_mgga_x_js18_lxc_pol_part21_v4rho3tau_5_v4rho3tau_6_v4rho3tau_7;
use part22::hyb_mgga_x_js18_lxc_pol_part22_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc;
use part23::hyb_mgga_x_js18_lxc_pol_part23_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9__etc;
use part24::hyb_mgga_x_js18_lxc_pol_part24_v4rho2sigma2_11;
use part25::hyb_mgga_x_js18_lxc_pol_part25_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc;
use part26::hyb_mgga_x_js18_lxc_pol_part26_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc;
use part27::hyb_mgga_x_js18_lxc_pol_part27_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc;
use part28::hyb_mgga_x_js18_lxc_pol_part28_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2sigmatau_14_v4rho2_etc;
use part29::hyb_mgga_x_js18_lxc_pol_part29_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
use part30::hyb_mgga_x_js18_lxc_pol_part30_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc;
use part31::hyb_mgga_x_js18_lxc_pol_part31_v4rhosigma3_10_v4rhosigma3_11_v4rhosigma3_12_v4rhosigma3_13__etc;
use part32::hyb_mgga_x_js18_lxc_pol_part32_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc;
use part33::hyb_mgga_x_js18_lxc_pol_part33_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc;
use part34::hyb_mgga_x_js18_lxc_pol_part34_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3__etc;
use part35::hyb_mgga_x_js18_lxc_pol_part35_v4rhotau3_v4sigma4_v4sigma3lapl;
use part36::hyb_mgga_x_js18_lxc_pol_part36_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau;
use part37::hyb_mgga_x_js18_lxc_pol_part37_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4_etc;

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_js18_lxc_pol(
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
    param_hyb_coeff_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    hyb_mgga_x_js18_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part1_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part2_v3rho3(rho, sigma, lapl, tau, v3rho3, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part3_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2_v3rho2sigma_3_v3rh_etc(rho, sigma, lapl, tau, v3rho2sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part4_v3rho2sigma_5_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8(rho, sigma, lapl, tau, v3rho2sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part5_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2lapl, v3rho2tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part6_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part7_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part8_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3sigmalaplta_etc(rho, sigma, lapl, tau, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part9_v4rho4_0(rho, sigma, lapl, tau, v4rho4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part10_v4rho4_1(rho, sigma, lapl, tau, v4rho4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part11_v4rho4_2(rho, sigma, lapl, tau, v4rho4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part12_v4rho4_3(rho, sigma, lapl, tau, v4rho4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part13_v4rho4_4(rho, sigma, lapl, tau, v4rho4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part14_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part15_v4rho3sigma_3_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part16_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part17_v4rho3sigma_6_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part18_v4rho3sigma_8_v4rho3sigma_9_v4rho3sigma_10(rho, sigma, lapl, tau, v4rho3sigma, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part19_v4rho3sigma_11_v4rho3lapl_v4rho3tau_0_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, v4rho3tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part20_v4rho3tau_2_v4rho3tau_3_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part21_v4rho3tau_5_v4rho3tau_6_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part22_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3__etc(rho, sigma, lapl, tau, v4rho2sigma2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part23_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9__etc(rho, sigma, lapl, tau, v4rho2sigma2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part24_v4rho2sigma2_11(rho, sigma, lapl, tau, v4rho2sigma2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part25_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part26_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part27_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part28_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2sigmatau_14_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmatau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part29_v4rho2lapl2_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part30_v4rhosigma3_0_v4rhosigma3_1_v4rhosigma3_2_v4rhosigma3_3_v4rh_etc(rho, sigma, lapl, tau, v4rhosigma3, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part31_v4rhosigma3_10_v4rhosigma3_11_v4rhosigma3_12_v4rhosigma3_13__etc(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part32_v4rhosigma2tau_0_v4rhosigma2tau_1_v4rhosigma2tau_2_v4rhosigm_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part33_v4rhosigma2tau_12_v4rhosigma2tau_13_v4rhosigma2tau_14_v4rhos_etc(rho, sigma, lapl, tau, v4rhosigma2tau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part34_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmatau2_v4rholapl3__etc(rho, sigma, lapl, tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part35_v4rhotau3_v4sigma4_v4sigma3lapl(rho, sigma, lapl, tau, v4rhotau3, v4sigma4, v4sigma3lapl, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part36_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau(rho, sigma, lapl, tau, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
    hyb_mgga_x_js18_lxc_pol_part37_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2_v4_etc(rho, sigma, lapl, tau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_hyb_coeff_0, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
