//! MGGA_C_KCISK lxc pol kernel — lxc_pol (nested-by-output, 51 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use libxc_rkernel_mgga_c_kcisk_p0::mgga_c_kcisk_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau;
use libxc_rkernel_mgga_c_kcisk_p0::mgga_c_kcisk_lxc_pol_part1_v2rho2;
use libxc_rkernel_mgga_c_kcisk_p0::mgga_c_kcisk_lxc_pol_part2_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;
use libxc_rkernel_mgga_c_kcisk_p1::mgga_c_kcisk_lxc_pol_part3_v3rho3_0;
use libxc_rkernel_mgga_c_kcisk_p2::mgga_c_kcisk_lxc_pol_part4_v3rho3_1;
use libxc_rkernel_mgga_c_kcisk_p2::mgga_c_kcisk_lxc_pol_part5_v3rho3_2;
use libxc_rkernel_mgga_c_kcisk_p3::mgga_c_kcisk_lxc_pol_part6_v3rho3_3;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part7_v3rho2sigma_0_v3rho2sigma_1;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part8_v3rho2sigma_2;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part9_v3rho2sigma_3_v3rho2sigma_4;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part10_v3rho2sigma_5;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part11_v3rho2sigma_6_v3rho2sigma_7;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part12_v3rho2sigma_8;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part13_v3rho2lapl_v3rho2tau;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part14_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part15_v4rho4_0;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part16_v4rho4_1;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part17_v4rho4_2;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part18_v4rho4_3;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part19_v4rho4_4;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part20_v4rho3sigma_0;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part21_v4rho3sigma_1;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part22_v4rho3sigma_2;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part24_v4rho3sigma_4;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part27_v4rho3sigma_7;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part29_v4rho3sigma_9;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part30_v4rho3sigma_10;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part31_v4rho3sigma_11_v4rho3lapl;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part32_v4rho3tau_0_v4rho3tau_1;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part33_v4rho3tau_2;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part34_v4rho3tau_3;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part35_v4rho3tau_4;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part36_v4rho3tau_5;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part37_v4rho3tau_6_v4rho3tau_7;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part38_v4rho2sigma2_0_v4rho2sigma2_1;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part39_v4rho2sigma2_2_v4rho2sigma2_3_v4rho2sigma2_4_v4rho2sigma2_5;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part40_v4rho2sigma2_6;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part41_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9_v4rho2sigma2_10;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part42_v4rho2sigma2_11;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part43_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part44_v4rho2sigma2_16_v4rho2sigma2_17_v4rho2sigmalapl;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part45_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc;
use libxc_rkernel_mgga_c_kcisk_p4::mgga_c_kcisk_lxc_pol_part46_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc;
use libxc_rkernel_mgga_c_kcisk_p5::mgga_c_kcisk_lxc_pol_part47_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2sigmatau_14_v4rho2_etc;
use libxc_rkernel_mgga_c_kcisk_p5::mgga_c_kcisk_lxc_pol_part48_v4rho2lapl2_v4rho2lapltau_v4rho2tau2;
use libxc_rkernel_mgga_c_kcisk_p5::mgga_c_kcisk_lxc_pol_part49_v4rhosigma3_v4rhosigma2lapl;
use libxc_rkernel_mgga_c_kcisk_p5::mgga_c_kcisk_lxc_pol_part50_v4rhosigma2tau_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmat_etc;

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_kcisk_lxc_pol(
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
    mgga_c_kcisk_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part1_v2rho2(rho, sigma, lapl, tau, v2rho2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part2_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part3_v3rho3_0(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part4_v3rho3_1(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part5_v3rho3_2(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part6_v3rho3_3(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part7_v3rho2sigma_0_v3rho2sigma_1(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part8_v3rho2sigma_2(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part9_v3rho2sigma_3_v3rho2sigma_4(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part10_v3rho2sigma_5(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part11_v3rho2sigma_6_v3rho2sigma_7(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part12_v3rho2sigma_8(rho, sigma, lapl, tau, v3rho2sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part13_v3rho2lapl_v3rho2tau(rho, sigma, lapl, tau, v3rho2lapl, v3rho2tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part14_v3rhosigma2_v3rhosigmalapl_v3rhosigmatau_v3rholapl2_v3rholap_etc(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part15_v4rho4_0(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part16_v4rho4_1(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part17_v4rho4_2(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part18_v4rho4_3(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part19_v4rho4_4(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part20_v4rho3sigma_0(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part21_v4rho3sigma_1(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part22_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part24_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part27_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part29_v4rho3sigma_9(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part30_v4rho3sigma_10(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part31_v4rho3sigma_11_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part32_v4rho3tau_0_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part33_v4rho3tau_2(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part34_v4rho3tau_3(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part35_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part36_v4rho3tau_5(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part37_v4rho3tau_6_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part38_v4rho2sigma2_0_v4rho2sigma2_1(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part39_v4rho2sigma2_2_v4rho2sigma2_3_v4rho2sigma2_4_v4rho2sigma2_5(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part40_v4rho2sigma2_6(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part41_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9_v4rho2sigma2_10(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part42_v4rho2sigma2_11(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part43_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_etc(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part44_v4rho2sigma2_16_v4rho2sigma2_17_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part45_v4rho2sigmatau_0_v4rho2sigmatau_1_v4rho2sigmatau_2_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part46_v4rho2sigmatau_6_v4rho2sigmatau_7_v4rho2sigmatau_8_v4rho2sig_etc(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part47_v4rho2sigmatau_12_v4rho2sigmatau_13_v4rho2sigmatau_14_v4rho2_etc(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part48_v4rho2lapl2_v4rho2lapltau_v4rho2tau2(rho, sigma, lapl, tau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part49_v4rhosigma3_v4rhosigma2lapl(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, dens_threshold, zeta_threshold);
    mgga_c_kcisk_lxc_pol_part50_v4rhosigma2tau_v4rhosigmalapl2_v4rhosigmalapltau_v4rhosigmat_etc(rho, sigma, lapl, tau, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, dens_threshold, zeta_threshold);
}
