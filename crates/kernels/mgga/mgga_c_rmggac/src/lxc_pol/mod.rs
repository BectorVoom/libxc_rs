//! MGGA_C_RMGGAC lxc pol kernel — lxc_pol (nested-by-output, 104 parts).
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
mod part65;
mod part66;
mod part67;
mod part68;
mod part69;
mod part70;
mod part71;
mod part72;
mod part73;
mod part74;
mod part75;
mod part76;
mod part77;
mod part78;
mod part79;
mod part80;
mod part81;
mod part82;
mod part83;
mod part84;
mod part85;
mod part86;
mod part87;
mod part88;
mod part89;
mod part90;
mod part91;
mod part92;
mod part93;
mod part94;
mod part95;
mod part96;
mod part97;
mod part98;
mod part99;
mod part100;
mod part101;
mod part102;
mod part103;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

use part0::mgga_c_rmggac_lxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2_v2rhosigma_v2rholapl_v2rhot_etc;
use part1::mgga_c_rmggac_lxc_pol_part1_v3rho3;
use part2::mgga_c_rmggac_lxc_pol_part2_v3rho2sigma_v3rho2lapl;
use part3::mgga_c_rmggac_lxc_pol_part3_v3rho2tau;
use part4::mgga_c_rmggac_lxc_pol_part4_v3rhosigma2_v3rhosigmalapl;
use part5::mgga_c_rmggac_lxc_pol_part5_v3rhosigmatau_v3rholapl2_v3rholapltau;
use part6::mgga_c_rmggac_lxc_pol_part6_v3rhotau2_v3sigma3_v3sigma2lapl_v3sigma2tau_v3sigmalapl2_v3s_etc;
use part7::mgga_c_rmggac_lxc_pol_part7_v3sigmatau2_v3lapl3_v3lapl2tau_v3lapltau2_v3tau3;
use part8::mgga_c_rmggac_lxc_pol_part8_v4rho4;
use part9::mgga_c_rmggac_lxc_pol_part9_v4rho3sigma_0;
use part10::mgga_c_rmggac_lxc_pol_part10_v4rho3sigma_1;
use part11::mgga_c_rmggac_lxc_pol_part11_v4rho3sigma_2;
use part12::mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3;
use part13::mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4;
use part14::mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5;
use part15::mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6;
use part16::mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7;
use part17::mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8;
use part18::mgga_c_rmggac_lxc_pol_part18_v4rho3sigma_9;
use part19::mgga_c_rmggac_lxc_pol_part19_v4rho3sigma_10;
use part20::mgga_c_rmggac_lxc_pol_part20_v4rho3sigma_11_v4rho3lapl;
use part21::mgga_c_rmggac_lxc_pol_part21_v4rho3tau_0_v4rho3tau_1;
use part22::mgga_c_rmggac_lxc_pol_part22_v4rho3tau_2;
use part23::mgga_c_rmggac_lxc_pol_part23_v4rho3tau_3;
use part24::mgga_c_rmggac_lxc_pol_part24_v4rho3tau_4;
use part25::mgga_c_rmggac_lxc_pol_part25_v4rho3tau_5_v4rho3tau_6;
use part26::mgga_c_rmggac_lxc_pol_part26_v4rho3tau_7;
use part27::mgga_c_rmggac_lxc_pol_part27_v4rho2sigma2_0;
use part28::mgga_c_rmggac_lxc_pol_part28_v4rho2sigma2_1;
use part29::mgga_c_rmggac_lxc_pol_part29_v4rho2sigma2_2;
use part30::mgga_c_rmggac_lxc_pol_part30_v4rho2sigma2_3;
use part31::mgga_c_rmggac_lxc_pol_part31_v4rho2sigma2_4;
use part32::mgga_c_rmggac_lxc_pol_part32_v4rho2sigma2_5;
use part33::mgga_c_rmggac_lxc_pol_part33_v4rho2sigma2_6;
use part34::mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7;
use part35::mgga_c_rmggac_lxc_pol_part35_v4rho2sigma2_8;
use part36::mgga_c_rmggac_lxc_pol_part36_v4rho2sigma2_9;
use part37::mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10;
use part38::mgga_c_rmggac_lxc_pol_part38_v4rho2sigma2_11;
use part39::mgga_c_rmggac_lxc_pol_part39_v4rho2sigma2_12;
use part40::mgga_c_rmggac_lxc_pol_part40_v4rho2sigma2_13;
use part41::mgga_c_rmggac_lxc_pol_part41_v4rho2sigma2_14;
use part42::mgga_c_rmggac_lxc_pol_part42_v4rho2sigma2_15;
use part43::mgga_c_rmggac_lxc_pol_part43_v4rho2sigma2_16;
use part44::mgga_c_rmggac_lxc_pol_part44_v4rho2sigma2_17_v4rho2sigmalapl;
use part45::mgga_c_rmggac_lxc_pol_part45_v4rho2sigmatau_0;
use part46::mgga_c_rmggac_lxc_pol_part46_v4rho2sigmatau_1;
use part47::mgga_c_rmggac_lxc_pol_part47_v4rho2sigmatau_2;
use part48::mgga_c_rmggac_lxc_pol_part48_v4rho2sigmatau_3;
use part49::mgga_c_rmggac_lxc_pol_part49_v4rho2sigmatau_4;
use part50::mgga_c_rmggac_lxc_pol_part50_v4rho2sigmatau_5;
use part51::mgga_c_rmggac_lxc_pol_part51_v4rho2sigmatau_6;
use part52::mgga_c_rmggac_lxc_pol_part52_v4rho2sigmatau_7;
use part53::mgga_c_rmggac_lxc_pol_part53_v4rho2sigmatau_8;
use part54::mgga_c_rmggac_lxc_pol_part54_v4rho2sigmatau_9;
use part55::mgga_c_rmggac_lxc_pol_part55_v4rho2sigmatau_10;
use part56::mgga_c_rmggac_lxc_pol_part56_v4rho2sigmatau_11;
use part57::mgga_c_rmggac_lxc_pol_part57_v4rho2sigmatau_12;
use part58::mgga_c_rmggac_lxc_pol_part58_v4rho2sigmatau_13;
use part59::mgga_c_rmggac_lxc_pol_part59_v4rho2sigmatau_14;
use part60::mgga_c_rmggac_lxc_pol_part60_v4rho2sigmatau_15;
use part61::mgga_c_rmggac_lxc_pol_part61_v4rho2sigmatau_16;
use part62::mgga_c_rmggac_lxc_pol_part62_v4rho2sigmatau_17_v4rho2lapl2_v4rho2lapltau_v4rho2tau2_0;
use part63::mgga_c_rmggac_lxc_pol_part63_v4rho2tau2_1_v4rho2tau2_2_v4rho2tau2_3;
use part64::mgga_c_rmggac_lxc_pol_part64_v4rho2tau2_4_v4rho2tau2_5_v4rho2tau2_6;
use part65::mgga_c_rmggac_lxc_pol_part65_v4rho2tau2_7_v4rho2tau2_8_v4rhosigma3_0;
use part66::mgga_c_rmggac_lxc_pol_part66_v4rhosigma3_1_v4rhosigma3_2;
use part67::mgga_c_rmggac_lxc_pol_part67_v4rhosigma3_3;
use part68::mgga_c_rmggac_lxc_pol_part68_v4rhosigma3_4;
use part69::mgga_c_rmggac_lxc_pol_part69_v4rhosigma3_5_v4rhosigma3_6;
use part70::mgga_c_rmggac_lxc_pol_part70_v4rhosigma3_7;
use part71::mgga_c_rmggac_lxc_pol_part71_v4rhosigma3_8_v4rhosigma3_9;
use part72::mgga_c_rmggac_lxc_pol_part72_v4rhosigma3_10_v4rhosigma3_11;
use part73::mgga_c_rmggac_lxc_pol_part73_v4rhosigma3_12_v4rhosigma3_13;
use part74::mgga_c_rmggac_lxc_pol_part74_v4rhosigma3_14;
use part75::mgga_c_rmggac_lxc_pol_part75_v4rhosigma3_15_v4rhosigma3_16;
use part76::mgga_c_rmggac_lxc_pol_part76_v4rhosigma3_17;
use part77::mgga_c_rmggac_lxc_pol_part77_v4rhosigma3_18_v4rhosigma3_19_v4rhosigma2lapl;
use part78::mgga_c_rmggac_lxc_pol_part78_v4rhosigma2tau_0_v4rhosigma2tau_1;
use part79::mgga_c_rmggac_lxc_pol_part79_v4rhosigma2tau_2;
use part80::mgga_c_rmggac_lxc_pol_part80_v4rhosigma2tau_3;
use part81::mgga_c_rmggac_lxc_pol_part81_v4rhosigma2tau_4_v4rhosigma2tau_5;
use part82::mgga_c_rmggac_lxc_pol_part82_v4rhosigma2tau_6_v4rhosigma2tau_7;
use part83::mgga_c_rmggac_lxc_pol_part83_v4rhosigma2tau_8;
use part84::mgga_c_rmggac_lxc_pol_part84_v4rhosigma2tau_9;
use part85::mgga_c_rmggac_lxc_pol_part85_v4rhosigma2tau_10_v4rhosigma2tau_11;
use part86::mgga_c_rmggac_lxc_pol_part86_v4rhosigma2tau_12_v4rhosigma2tau_13;
use part87::mgga_c_rmggac_lxc_pol_part87_v4rhosigma2tau_14;
use part88::mgga_c_rmggac_lxc_pol_part88_v4rhosigma2tau_15;
use part89::mgga_c_rmggac_lxc_pol_part89_v4rhosigma2tau_16_v4rhosigma2tau_17;
use part90::mgga_c_rmggac_lxc_pol_part90_v4rhosigma2tau_18_v4rhosigma2tau_19;
use part91::mgga_c_rmggac_lxc_pol_part91_v4rhosigma2tau_20;
use part92::mgga_c_rmggac_lxc_pol_part92_v4rhosigma2tau_21;
use part93::mgga_c_rmggac_lxc_pol_part93_v4rhosigma2tau_22_v4rhosigma2tau_23_v4rhosigmalapl2_v4rhosig_etc;
use part94::mgga_c_rmggac_lxc_pol_part94_v4rhosigmatau2_0_v4rhosigmatau2_1_v4rhosigmatau2_2;
use part95::mgga_c_rmggac_lxc_pol_part95_v4rhosigmatau2_3_v4rhosigmatau2_4_v4rhosigmatau2_5;
use part96::mgga_c_rmggac_lxc_pol_part96_v4rhosigmatau2_6_v4rhosigmatau2_7_v4rhosigmatau2_8;
use part97::mgga_c_rmggac_lxc_pol_part97_v4rhosigmatau2_9_v4rhosigmatau2_10_v4rhosigmatau2_11;
use part98::mgga_c_rmggac_lxc_pol_part98_v4rhosigmatau2_12_v4rhosigmatau2_13_v4rhosigmatau2_14;
use part99::mgga_c_rmggac_lxc_pol_part99_v4rhosigmatau2_15_v4rhosigmatau2_16_v4rhosigmatau2_17;
use part100::mgga_c_rmggac_lxc_pol_part100_v4rholapl3_v4rholapl2tau_v4rholapltau2_v4rhotau3_v4sigma4_v4_etc;
use part101::mgga_c_rmggac_lxc_pol_part101_v4sigma3tau_v4sigma2lapl2_v4sigma2lapltau;
use part102::mgga_c_rmggac_lxc_pol_part102_v4sigma2tau2_v4sigmalapl3_v4sigmalapl2tau_v4sigmalapltau2;
use part103::mgga_c_rmggac_lxc_pol_part103_v4sigmatau3_v4lapl4_v4lapl3tau_v4lapl2tau2_v4lapltau3_v4tau4;

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
