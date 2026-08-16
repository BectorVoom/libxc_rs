//! GGA_C_SOGGA11 lxc pol kernel — lxc_pol (nested-by-output, 10 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use part0::gga_c_sogga11_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc;
use part1::gga_c_sogga11_lxc_pol_part1_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2;
use part2::gga_c_sogga11_lxc_pol_part2_v4rho3sigma_3;
use part3::gga_c_sogga11_lxc_pol_part3_v4rho3sigma_4;
use part4::gga_c_sogga11_lxc_pol_part4_v4rho3sigma_5;
use part5::gga_c_sogga11_lxc_pol_part5_v4rho3sigma_6;
use part6::gga_c_sogga11_lxc_pol_part6_v4rho3sigma_7;
use part7::gga_c_sogga11_lxc_pol_part7_v4rho3sigma_8;
use part8::gga_c_sogga11_lxc_pol_part8_v4rho3sigma_9_v4rho3sigma_10_v4rho3sigma_11;
use part9::gga_c_sogga11_lxc_pol_part9_v4rho2sigma2_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sogga11_lxc_pol(
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
    param_sogga11_a_0: f64,
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_0: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_sogga11_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, v4rho4, param_sogga11_a_0, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_0, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part1_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part2_v4rho3sigma_3(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part3_v4rho3sigma_4(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part4_v4rho3sigma_5(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part5_v4rho3sigma_6(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part6_v4rho3sigma_7(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part7_v4rho3sigma_8(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part8_v4rho3sigma_9_v4rho3sigma_10_v4rho3sigma_11(rho, sigma, v4rho3sigma, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
    gga_c_sogga11_lxc_pol_part9_v4rho2sigma2_v4rhosigma3_v4sigma4(rho, sigma, v4rho2sigma2, v4rhosigma3, v4sigma4, param_sogga11_a_1, param_sogga11_a_2, param_sogga11_a_3, param_sogga11_a_4, param_sogga11_a_5, param_sogga11_b_1, param_sogga11_b_2, param_sogga11_b_3, param_sogga11_b_4, param_sogga11_b_5, dens_threshold, zeta_threshold);
}
