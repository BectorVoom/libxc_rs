//! MGGA_C_M06L lxc pol kernel — lxc_pol (nested-by-output, 14 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_m06l_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_m06l_lxc_pol_part1_v3rho3;
use part2::mgga_c_m06l_lxc_pol_part2_v3rho2sigma_v3rho2lapl_v3rho2tau_v3rhosigma2_v3rhosigmalapl;
use part3::mgga_c_m06l_lxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc;
use part4::mgga_c_m06l_lxc_pol_part4_v4rho4_1;
use part5::mgga_c_m06l_lxc_pol_part5_v4rho4_2;
use part6::mgga_c_m06l_lxc_pol_part6_v4rho4_3;
use part7::mgga_c_m06l_lxc_pol_part7_v4rho4_4;
use part8::mgga_c_m06l_lxc_pol_part8_v4rho3sigma_v4rho3lapl;
use part9::mgga_c_m06l_lxc_pol_part9_v4rho3tau;
use part10::mgga_c_m06l_lxc_pol_part10_v4rho2sigma2_v4rho2sigmalapl;
use part11::mgga_c_m06l_lxc_pol_part11_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau;
use part12::mgga_c_m06l_lxc_pol_part12_v4rho2tau2_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhos_etc;
use part13::mgga_c_m06l_lxc_pol_part13_v4rhosigmatau2_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhot_etc;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m06l_lxc_pol(
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
    param_Fermi_D_cnst: f64,
    param_alpha_ab: f64,
    param_alpha_ss: f64,
    param_cab_0: f64,
    param_cab_1: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_css_0: f64,
    param_css_1: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_dab_0: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    param_dss_0: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_gamma_ab: f64,
    param_gamma_ss: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_m06l_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part1_v3rho3(rho, sigma, lapl, tau, v3rho3, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part2_v3rho2sigma_v3rho2lapl_v3rho2tau_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2, v3rhosigmalapl, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part3_v3rhosigmatau_v3rholapl2_v3rholapltau_v3rhotau2_v3sigma3_v3s_etc(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, v4rho4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part4_v4rho4_1(rho, sigma, lapl, tau, v4rho4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part5_v4rho4_2(rho, sigma, lapl, tau, v4rho4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part6_v4rho4_3(rho, sigma, lapl, tau, v4rho4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part7_v4rho4_4(rho, sigma, lapl, tau, v4rho4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_0, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part8_v4rho3sigma_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part9_v4rho3tau(rho, sigma, lapl, tau, v4rho3tau, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part10_v4rho2sigma2_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part11_v4rho2sigmatau_v4rho2lapl2_v4rho2lapltau(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part12_v4rho2tau2_v4rhosigma3_v4rhosigma2lapl_v4rhosigma2tau_v4rhos_etc(rho, sigma, lapl, tau, v4rho2tau2, v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
    mgga_c_m06l_lxc_pol_part13_v4rhosigmatau2_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhot_etc(rho, sigma, lapl, tau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, param_Fermi_D_cnst, param_alpha_ab, param_alpha_ss, param_cab_1, param_cab_2, param_cab_3, param_cab_4, param_css_0, param_css_1, param_css_2, param_css_3, param_css_4, param_dab_0, param_dab_1, param_dab_2, param_dab_3, param_dab_4, param_dab_5, param_dss_0, param_dss_1, param_dss_2, param_dss_3, param_dss_4, param_dss_5, param_gamma_ab, param_gamma_ss, dens_threshold, zeta_threshold);
}
