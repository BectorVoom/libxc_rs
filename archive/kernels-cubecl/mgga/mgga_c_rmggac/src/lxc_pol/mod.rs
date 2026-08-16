//! MGGA_C_RMGGAC lxc pol kernel — lxc_pol (nested-by-output, 104 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]


use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part1_v3rho3;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part2_v3rho2sigma_v3rho2lapl;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part3_v3rho2tau;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part4_v3rhosigma2_v3rhosigmalapl;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part5_v3rhosigmatau_v3rholapl2_v3rholapltau;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part6_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part7_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part8_v4rho4;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part9_v4rho3sigma_0;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part10_v4rho3sigma_1;
use libxc_kernel_mgga_c_rmggac_p0::mgga_c_rmggac_lxc_pol_part11_v4rho3sigma_2;
use libxc_kernel_mgga_c_rmggac_p1::mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3;
use libxc_kernel_mgga_c_rmggac_p2::mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4;
use libxc_kernel_mgga_c_rmggac_p3::mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5;
use libxc_kernel_mgga_c_rmggac_p4::mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6;
use libxc_kernel_mgga_c_rmggac_p5::mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7;
use libxc_kernel_mgga_c_rmggac_p6::mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part18_v4rho3sigma_9;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part19_v4rho3sigma_10;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part20_v4rho3sigma_11_v4rho3lapl;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part21_v4rho3tau_0_v4rho3tau_1;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part22_v4rho3tau_2;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part23_v4rho3tau_3;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part24_v4rho3tau_4;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part25_v4rho3tau_5_v4rho3tau_6;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part26_v4rho3tau_7;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part27_v4rho2sigma2_0;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part28_v4rho2sigma2_1;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part29_v4rho2sigma2_2;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part30_v4rho2sigma2_3;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part31_v4rho2sigma2_4;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part32_v4rho2sigma2_5;
use libxc_kernel_mgga_c_rmggac_p7::mgga_c_rmggac_lxc_pol_part33_v4rho2sigma2_6;
use libxc_kernel_mgga_c_rmggac_p8::mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7;
use libxc_kernel_mgga_c_rmggac_p9::mgga_c_rmggac_lxc_pol_part35_v4rho2sigma2_8;
use libxc_kernel_mgga_c_rmggac_p9::mgga_c_rmggac_lxc_pol_part36_v4rho2sigma2_9;
use libxc_kernel_mgga_c_rmggac_p10::mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part38_v4rho2sigma2_11;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part39_v4rho2sigma2_12;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part40_v4rho2sigma2_13;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part41_v4rho2sigma2_14;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part42_v4rho2sigma2_15;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part43_v4rho2sigma2_16;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part44_v4rho2sigma2_17_v4rho2sigmalapl;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part45_v4rho2sigmatau_0;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part46_v4rho2sigmatau_1;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part47_v4rho2sigmatau_2;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part48_v4rho2sigmatau_3;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part49_v4rho2sigmatau_4;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part50_v4rho2sigmatau_5;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part51_v4rho2sigmatau_6;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part52_v4rho2sigmatau_7;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part53_v4rho2sigmatau_8;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part54_v4rho2sigmatau_9;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part55_v4rho2sigmatau_10;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part56_v4rho2sigmatau_11;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part57_v4rho2sigmatau_12;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part58_v4rho2sigmatau_13;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part59_v4rho2sigmatau_14;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part60_v4rho2sigmatau_15;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part61_v4rho2sigmatau_16;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part62_v4rho2sigmatau_17_v4rho2lapl2_v4rho2lapltau_v4rho2tau2_0;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part63_v4rho2tau2_1_v4rho2tau2_2_v4rho2tau2_3;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part64_v4rho2tau2_4_v4rho2tau2_5_v4rho2tau2_6;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part65_v4rho2tau2_7_v4rho2tau2_8_v4rhosigma3_0;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part66_v4rhosigma3_1_v4rhosigma3_2;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part67_v4rhosigma3_3;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part68_v4rhosigma3_4;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part69_v4rhosigma3_5_v4rhosigma3_6;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part70_v4rhosigma3_7;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part71_v4rhosigma3_8_v4rhosigma3_9;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part72_v4rhosigma3_10_v4rhosigma3_11;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part73_v4rhosigma3_12_v4rhosigma3_13;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part74_v4rhosigma3_14;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part75_v4rhosigma3_15_v4rhosigma3_16;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part76_v4rhosigma3_17;
use libxc_kernel_mgga_c_rmggac_p11::mgga_c_rmggac_lxc_pol_part77_v4rhosigma3_18_v4rhosigma3_19_v4rhosigma2lapl;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part78_v4rhosigma2tau_0_v4rhosigma2tau_1;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part79_v4rhosigma2tau_2;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part80_v4rhosigma2tau_3;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part81_v4rhosigma2tau_4_v4rhosigma2tau_5;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part82_v4rhosigma2tau_6_v4rhosigma2tau_7;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part83_v4rhosigma2tau_8;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part84_v4rhosigma2tau_9;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part85_v4rhosigma2tau_10_v4rhosigma2tau_11;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part86_v4rhosigma2tau_12_v4rhosigma2tau_13;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part87_v4rhosigma2tau_14;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part88_v4rhosigma2tau_15;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part89_v4rhosigma2tau_16_v4rhosigma2tau_17;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part90_v4rhosigma2tau_18_v4rhosigma2tau_19;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part91_v4rhosigma2tau_20;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part92_v4rhosigma2tau_21;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part93_v4rhosigma2tau_22_v4rhosigma2tau_23_v4rhosigmalapl2_v4rhosig_etc;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part94_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part95_v4rhosigmatau2_3_v4rhosigmatau2_4_v4rhosigmatau2_5;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part96_v4rhosigmatau2_6_v4rhosigmatau2_7_v4rhosigmatau2_8;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part97_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part98_v4rhosigmatau2_12_v4rhosigmatau2_13_v4rhosigmatau2_14;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part99_v4rhosigmatau2_15_v4rhosigmatau2_16_v4rhosigmatau2_17;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part100_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4_v4_etc;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part101_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part102_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2;
use libxc_kernel_mgga_c_rmggac_p12::mgga_c_rmggac_lxc_pol_part103_v4sigmatau3_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_rmggac_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part1_v3rho3(rho, sigma, lapl, tau, v3rho3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part2_v3rho2sigma_v3rho2lapl(rho, sigma, lapl, tau, v3rho2sigma, v3rho2lapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part3_v3rho2tau(rho, sigma, lapl, tau, v3rho2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part4_v3rhosigma2_v3rhosigmalapl(rho, sigma, lapl, tau, v3rhosigma2, v3rhosigmalapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part5_v3rhosigmatau_v3rholapl2_v3rholapltau(rho, sigma, lapl, tau, v3rhosigmatau, v3rholapl2, v3rholapltau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part6_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc(rho, sigma, lapl, tau, v3rhotau2, v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part7_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3(rho, sigma, lapl, tau, v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part8_v4rho4(rho, sigma, lapl, tau, v4rho4, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part9_v4rho3sigma_0(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part10_v4rho3sigma_1(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part11_v4rho3sigma_2(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part18_v4rho3sigma_9(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part19_v4rho3sigma_10(rho, sigma, lapl, tau, v4rho3sigma, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part20_v4rho3sigma_11_v4rho3lapl(rho, sigma, lapl, tau, v4rho3sigma, v4rho3lapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part21_v4rho3tau_0_v4rho3tau_1(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part22_v4rho3tau_2(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part23_v4rho3tau_3(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part24_v4rho3tau_4(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part25_v4rho3tau_5_v4rho3tau_6(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part26_v4rho3tau_7(rho, sigma, lapl, tau, v4rho3tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part27_v4rho2sigma2_0(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part28_v4rho2sigma2_1(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part29_v4rho2sigma2_2(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part30_v4rho2sigma2_3(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part31_v4rho2sigma2_4(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part32_v4rho2sigma2_5(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part33_v4rho2sigma2_6(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part35_v4rho2sigma2_8(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part36_v4rho2sigma2_9(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part38_v4rho2sigma2_11(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part39_v4rho2sigma2_12(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part40_v4rho2sigma2_13(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part41_v4rho2sigma2_14(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part42_v4rho2sigma2_15(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part43_v4rho2sigma2_16(rho, sigma, lapl, tau, v4rho2sigma2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part44_v4rho2sigma2_17_v4rho2sigmalapl(rho, sigma, lapl, tau, v4rho2sigma2, v4rho2sigmalapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part45_v4rho2sigmatau_0(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part46_v4rho2sigmatau_1(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part47_v4rho2sigmatau_2(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part48_v4rho2sigmatau_3(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part49_v4rho2sigmatau_4(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part50_v4rho2sigmatau_5(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part51_v4rho2sigmatau_6(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part52_v4rho2sigmatau_7(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part53_v4rho2sigmatau_8(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part54_v4rho2sigmatau_9(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part55_v4rho2sigmatau_10(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part56_v4rho2sigmatau_11(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part57_v4rho2sigmatau_12(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part58_v4rho2sigmatau_13(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part59_v4rho2sigmatau_14(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part60_v4rho2sigmatau_15(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part61_v4rho2sigmatau_16(rho, sigma, lapl, tau, v4rho2sigmatau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part62_v4rho2sigmatau_17_v4rho2lapl2_v4rho2lapltau_v4rho2tau2_0(rho, sigma, lapl, tau, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part63_v4rho2tau2_1_v4rho2tau2_2_v4rho2tau2_3(rho, sigma, lapl, tau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part64_v4rho2tau2_4_v4rho2tau2_5_v4rho2tau2_6(rho, sigma, lapl, tau, v4rho2tau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part65_v4rho2tau2_7_v4rho2tau2_8_v4rhosigma3_0(rho, sigma, lapl, tau, v4rho2tau2, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part66_v4rhosigma3_1_v4rhosigma3_2(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part67_v4rhosigma3_3(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part68_v4rhosigma3_4(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part69_v4rhosigma3_5_v4rhosigma3_6(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part70_v4rhosigma3_7(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part71_v4rhosigma3_8_v4rhosigma3_9(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part72_v4rhosigma3_10_v4rhosigma3_11(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part73_v4rhosigma3_12_v4rhosigma3_13(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part74_v4rhosigma3_14(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part75_v4rhosigma3_15_v4rhosigma3_16(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part76_v4rhosigma3_17(rho, sigma, lapl, tau, v4rhosigma3, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part77_v4rhosigma3_18_v4rhosigma3_19_v4rhosigma2lapl(rho, sigma, lapl, tau, v4rhosigma3, v4rhosigma2lapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part78_v4rhosigma2tau_0_v4rhosigma2tau_1(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part79_v4rhosigma2tau_2(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part80_v4rhosigma2tau_3(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part81_v4rhosigma2tau_4_v4rhosigma2tau_5(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part82_v4rhosigma2tau_6_v4rhosigma2tau_7(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part83_v4rhosigma2tau_8(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part84_v4rhosigma2tau_9(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part85_v4rhosigma2tau_10_v4rhosigma2tau_11(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part86_v4rhosigma2tau_12_v4rhosigma2tau_13(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part87_v4rhosigma2tau_14(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part88_v4rhosigma2tau_15(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part89_v4rhosigma2tau_16_v4rhosigma2tau_17(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part90_v4rhosigma2tau_18_v4rhosigma2tau_19(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part91_v4rhosigma2tau_20(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part92_v4rhosigma2tau_21(rho, sigma, lapl, tau, v4rhosigma2tau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part93_v4rhosigma2tau_22_v4rhosigma2tau_23_v4rhosigmalapl2_v4rhosig_etc(rho, sigma, lapl, tau, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part94_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part95_v4rhosigmatau2_3_v4rhosigmatau2_4_v4rhosigmatau2_5(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part96_v4rhosigmatau2_6_v4rhosigmatau2_7_v4rhosigmatau2_8(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part97_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part98_v4rhosigmatau2_12_v4rhosigmatau2_13_v4rhosigmatau2_14(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part99_v4rhosigmatau2_15_v4rhosigmatau2_16_v4rhosigmatau2_17(rho, sigma, lapl, tau, v4rhosigmatau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part100_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4_v4_etc(rho, sigma, lapl, tau, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3, v4sigma4, v4sigma3lapl, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part101_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau(rho, sigma, lapl, tau, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part102_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2(rho, sigma, lapl, tau, v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, dens_threshold, zeta_threshold);
    mgga_c_rmggac_lxc_pol_part103_v4sigmatau3_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4(rho, sigma, lapl, tau, v4sigmatau3, v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4, dens_threshold, zeta_threshold);
}
