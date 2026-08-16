//! GGA_C_SG4 lxc pol kernel — lxc_pol (nested-by-output, 24 parts).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::gga_c_sg4_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3;
use part1::gga_c_sg4_lxc_pol_part1_v3rho2sigma_v3rhosigma2_v3sigma3;
use part2::gga_c_sg4_lxc_pol_part2_v4rho4;
use part3::gga_c_sg4_lxc_pol_part3_v4rho3sigma_0_v4rho3sigma_1;
use part4::gga_c_sg4_lxc_pol_part4_v4rho3sigma_2;
use part5::gga_c_sg4_lxc_pol_part5_v4rho3sigma_3;
use part6::gga_c_sg4_lxc_pol_part6_v4rho3sigma_4;
use part7::gga_c_sg4_lxc_pol_part7_v4rho3sigma_5;
use part8::gga_c_sg4_lxc_pol_part8_v4rho3sigma_6;
use part9::gga_c_sg4_lxc_pol_part9_v4rho3sigma_7;
use part10::gga_c_sg4_lxc_pol_part10_v4rho3sigma_8;
use part11::gga_c_sg4_lxc_pol_part11_v4rho3sigma_9_v4rho3sigma_10;
use part12::gga_c_sg4_lxc_pol_part12_v4rho3sigma_11_v4rho2sigma2_0;
use part13::gga_c_sg4_lxc_pol_part13_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3;
use part14::gga_c_sg4_lxc_pol_part14_v4rho2sigma2_4_v4rho2sigma2_5;
use part15::gga_c_sg4_lxc_pol_part15_v4rho2sigma2_6;
use part16::gga_c_sg4_lxc_pol_part16_v4rho2sigma2_7;
use part17::gga_c_sg4_lxc_pol_part17_v4rho2sigma2_8;
use part18::gga_c_sg4_lxc_pol_part18_v4rho2sigma2_9;
use part19::gga_c_sg4_lxc_pol_part19_v4rho2sigma2_10;
use part20::gga_c_sg4_lxc_pol_part20_v4rho2sigma2_11_v4rho2sigma2_12;
use part21::gga_c_sg4_lxc_pol_part21_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_15;
use part22::gga_c_sg4_lxc_pol_part22_v4rho2sigma2_16_v4rho2sigma2_17;
use part23::gga_c_sg4_lxc_pol_part23_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_sg4_lxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_sg4_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part1_v3rho2sigma_v3rhosigma2_v3sigma3(rho, sigma, v3rho2sigma, v3rhosigma2, v3sigma3, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part2_v4rho4(rho, sigma, v4rho4, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part3_v4rho3sigma_0_v4rho3sigma_1(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part4_v4rho3sigma_2(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part5_v4rho3sigma_3(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part6_v4rho3sigma_4(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part7_v4rho3sigma_5(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part8_v4rho3sigma_6(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part9_v4rho3sigma_7(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part10_v4rho3sigma_8(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part11_v4rho3sigma_9_v4rho3sigma_10(rho, sigma, v4rho3sigma, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part12_v4rho3sigma_11_v4rho2sigma2_0(rho, sigma, v4rho3sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part13_v4rho2sigma2_1_v4rho2sigma2_2_v4rho2sigma2_3(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part14_v4rho2sigma2_4_v4rho2sigma2_5(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part15_v4rho2sigma2_6(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part16_v4rho2sigma2_7(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part17_v4rho2sigma2_8(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part18_v4rho2sigma2_9(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part19_v4rho2sigma2_10(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part20_v4rho2sigma2_11_v4rho2sigma2_12(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part21_v4rho2sigma2_13_v4rho2sigma2_14_v4rho2sigma2_15(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part22_v4rho2sigma2_16_v4rho2sigma2_17(rho, sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_sg4_lxc_pol_part23_v4rhosigma3_v4sigma4(rho, sigma, v4rhosigma3, v4sigma4, dens_threshold, zeta_threshold);
}
