//! GGA_C_GAPLOC lxc pol kernel — lxc_pol (nested-by-output, 64 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use part0::gga_c_gaploc_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3;
use part1::gga_c_gaploc_lxc_pol_part1_v3rho2sigma;
use part2::gga_c_gaploc_lxc_pol_part2_v3rhosigma2;
use part3::gga_c_gaploc_lxc_pol_part3_v3sigma3;
use part4::gga_c_gaploc_lxc_pol_part4_v4rho4;
use part5::gga_c_gaploc_lxc_pol_part5_v4rho3sigma_0;
use part6::gga_c_gaploc_lxc_pol_part6_v4rho3sigma_1;
use part7::gga_c_gaploc_lxc_pol_part7_v4rho3sigma_2;
use part8::gga_c_gaploc_lxc_pol_part8_v4rho3sigma_3;
use part9::gga_c_gaploc_lxc_pol_part9_v4rho3sigma_4;
use part10::gga_c_gaploc_lxc_pol_part10_v4rho3sigma_5;
use part11::gga_c_gaploc_lxc_pol_part11_v4rho3sigma_6;
use part12::gga_c_gaploc_lxc_pol_part12_v4rho3sigma_7;
use part13::gga_c_gaploc_lxc_pol_part13_v4rho3sigma_8;
use part14::gga_c_gaploc_lxc_pol_part14_v4rho3sigma_9;
use part15::gga_c_gaploc_lxc_pol_part15_v4rho3sigma_10;
use part16::gga_c_gaploc_lxc_pol_part16_v4rho3sigma_11;
use part17::gga_c_gaploc_lxc_pol_part17_v4rho2sigma2_0;
use part18::gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1;
use part19::gga_c_gaploc_lxc_pol_part19_v4rho2sigma2_2;
use part20::gga_c_gaploc_lxc_pol_part20_v4rho2sigma2_3;
use part21::gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4;
use part22::gga_c_gaploc_lxc_pol_part22_v4rho2sigma2_5;
use part23::gga_c_gaploc_lxc_pol_part23_v4rho2sigma2_6;
use part24::gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7;
use part25::gga_c_gaploc_lxc_pol_part25_v4rho2sigma2_8;
use part26::gga_c_gaploc_lxc_pol_part26_v4rho2sigma2_9;
use part27::gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10;
use part28::gga_c_gaploc_lxc_pol_part28_v4rho2sigma2_11;
use part29::gga_c_gaploc_lxc_pol_part29_v4rho2sigma2_12;
use part30::gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13;
use part31::gga_c_gaploc_lxc_pol_part31_v4rho2sigma2_14;
use part32::gga_c_gaploc_lxc_pol_part32_v4rho2sigma2_15;
use part33::gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16;
use part34::gga_c_gaploc_lxc_pol_part34_v4rho2sigma2_17;
use part35::gga_c_gaploc_lxc_pol_part35_v4rhosigma3_0;
use part36::gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1;
use part37::gga_c_gaploc_lxc_pol_part37_v4rhosigma3_2;
use part38::gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3;
use part39::gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4;
use part40::gga_c_gaploc_lxc_pol_part40_v4rhosigma3_5;
use part41::gga_c_gaploc_lxc_pol_part41_v4rhosigma3_6;
use part42::gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7;
use part43::gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8;
use part44::gga_c_gaploc_lxc_pol_part44_v4rhosigma3_9;
use part45::gga_c_gaploc_lxc_pol_part45_v4rhosigma3_10;
use part46::gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11;
use part47::gga_c_gaploc_lxc_pol_part47_v4rhosigma3_12;
use part48::gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13;
use part49::gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14;
use part50::gga_c_gaploc_lxc_pol_part50_v4rhosigma3_15;
use part51::gga_c_gaploc_lxc_pol_part51_v4rhosigma3_16;
use part52::gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17;
use part53::gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18;
use part54::gga_c_gaploc_lxc_pol_part54_v4rhosigma3_19_v4sigma4_0;
use part55::gga_c_gaploc_lxc_pol_part55_v4sigma4_1_v4sigma4_2;
use part56::gga_c_gaploc_lxc_pol_part56_v4sigma4_3;
use part57::gga_c_gaploc_lxc_pol_part57_v4sigma4_4_v4sigma4_5;
use part58::gga_c_gaploc_lxc_pol_part58_v4sigma4_6;
use part59::gga_c_gaploc_lxc_pol_part59_v4sigma4_7;
use part60::gga_c_gaploc_lxc_pol_part60_v4sigma4_8_v4sigma4_9_v4sigma4_10;
use part61::gga_c_gaploc_lxc_pol_part61_v4sigma4_11;
use part62::gga_c_gaploc_lxc_pol_part62_v4sigma4_12;
use part63::gga_c_gaploc_lxc_pol_part63_v4sigma4_13_v4sigma4_14;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_gaploc_lxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_gaploc_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part1_v3rho2sigma(rho, sigma, v3rho2sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part2_v3rhosigma2(rho, sigma, v3rhosigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part3_v3sigma3(rho, sigma, v3sigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part4_v4rho4(rho, sigma, v4rho4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part5_v4rho3sigma_0(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part6_v4rho3sigma_1(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part7_v4rho3sigma_2(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part8_v4rho3sigma_3(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part9_v4rho3sigma_4(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part10_v4rho3sigma_5(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part11_v4rho3sigma_6(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part12_v4rho3sigma_7(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part13_v4rho3sigma_8(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part14_v4rho3sigma_9(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part15_v4rho3sigma_10(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part16_v4rho3sigma_11(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part17_v4rho2sigma2_0(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part19_v4rho2sigma2_2(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part20_v4rho2sigma2_3(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part22_v4rho2sigma2_5(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part23_v4rho2sigma2_6(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part25_v4rho2sigma2_8(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part26_v4rho2sigma2_9(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part28_v4rho2sigma2_11(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part29_v4rho2sigma2_12(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part31_v4rho2sigma2_14(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part32_v4rho2sigma2_15(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part34_v4rho2sigma2_17(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part35_v4rhosigma3_0(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part37_v4rhosigma3_2(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part40_v4rhosigma3_5(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part41_v4rhosigma3_6(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part44_v4rhosigma3_9(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part45_v4rhosigma3_10(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part47_v4rhosigma3_12(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part50_v4rhosigma3_15(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part51_v4rhosigma3_16(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18(rho, sigma, v4rhosigma3, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part54_v4rhosigma3_19_v4sigma4_0(rho, sigma, v4rhosigma3, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part55_v4sigma4_1_v4sigma4_2(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part56_v4sigma4_3(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part57_v4sigma4_4_v4sigma4_5(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part58_v4sigma4_6(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part59_v4sigma4_7(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part60_v4sigma4_8_v4sigma4_9_v4sigma4_10(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part61_v4sigma4_11(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part62_v4sigma4_12(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
    gga_c_gaploc_lxc_pol_part63_v4sigma4_13_v4sigma4_14(rho, sigma, v4sigma4, dens_threshold, zeta_threshold);
}
