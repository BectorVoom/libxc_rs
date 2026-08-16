//! GGA_X_LCGAU lxc pol kernel — lxc_pol (nested-by-output, 19 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

use part0::gga_x_lcgau_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2;
use part1::gga_x_lcgau_lxc_pol_part1_v3rho3;
use part2::gga_x_lcgau_lxc_pol_part2_v3rho2sigma;
use part3::gga_x_lcgau_lxc_pol_part3_v3rhosigma2;
use part4::gga_x_lcgau_lxc_pol_part4_v3sigma3_v4rho4_0;
use part5::gga_x_lcgau_lxc_pol_part5_v4rho4_1;
use part6::gga_x_lcgau_lxc_pol_part6_v4rho4_2;
use part7::gga_x_lcgau_lxc_pol_part7_v4rho4_3;
use part8::gga_x_lcgau_lxc_pol_part8_v4rho4_4_v4rho3sigma_0_v4rho3sigma_1;
use part9::gga_x_lcgau_lxc_pol_part9_v4rho3sigma_2_v4rho3sigma_3_v4rho3sigma_4;
use part10::gga_x_lcgau_lxc_pol_part10_v4rho3sigma_5;
use part11::gga_x_lcgau_lxc_pol_part11_v4rho3sigma_6_v4rho3sigma_7;
use part12::gga_x_lcgau_lxc_pol_part12_v4rho3sigma_8_v4rho3sigma_9_v4rho3sigma_10;
use part13::gga_x_lcgau_lxc_pol_part13_v4rho3sigma_11_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2__etc;
use part14::gga_x_lcgau_lxc_pol_part14_v4rho2sigma2_5_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8__etc;
use part15::gga_x_lcgau_lxc_pol_part15_v4rho2sigma2_11_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_etc;
use part16::gga_x_lcgau_lxc_pol_part16_v4rho2sigma2_17;
use part17::gga_x_lcgau_lxc_pol_part17_v4rhosigma3;
use part18::gga_x_lcgau_lxc_pol_part18_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lcgau_lxc_pol(
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
    param_hyb_coeff_2: f64,
    param_hyb_coeff_3: f64,
    param_hyb_omega_0: f64,
    param_hyb_omega_2: f64,
    param_hyb_omega_3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_lcgau_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part1_v3rho3(rho, sigma, v3rho3, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part2_v3rho2sigma(rho, sigma, v3rho2sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part3_v3rhosigma2(rho, sigma, v3rhosigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part4_v3sigma3_v4rho4_0(rho, sigma, v3sigma3, v4rho4, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part5_v4rho4_1(rho, sigma, v4rho4, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part6_v4rho4_2(rho, sigma, v4rho4, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part7_v4rho4_3(rho, sigma, v4rho4, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part8_v4rho4_4_v4rho3sigma_0_v4rho3sigma_1(rho, sigma, v4rho4, v4rho3sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part9_v4rho3sigma_2_v4rho3sigma_3_v4rho3sigma_4(rho, sigma, v4rho3sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part10_v4rho3sigma_5(rho, sigma, v4rho3sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part11_v4rho3sigma_6_v4rho3sigma_7(rho, sigma, v4rho3sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part12_v4rho3sigma_8_v4rho3sigma_9_v4rho3sigma_10(rho, sigma, v4rho3sigma, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part13_v4rho3sigma_11_v4rho2sigma2_0_v4rho2sigma2_1_v4rho2sigma2_2__etc(rho, sigma, v4rho3sigma, v4rho2sigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part14_v4rho2sigma2_5_v4rho2sigma2_6_v4rho2sigma2_7_v4rho2sigma2_8__etc(rho, sigma, v4rho2sigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part15_v4rho2sigma2_11_v4rho2sigma2_12_v4rho2sigma2_13_v4rho2sigma2_etc(rho, sigma, v4rho2sigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part16_v4rho2sigma2_17(rho, sigma, v4rho2sigma2, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part17_v4rhosigma3(rho, sigma, v4rhosigma3, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
    gga_x_lcgau_lxc_pol_part18_v4sigma4(rho, sigma, v4sigma4, param_hyb_coeff_2, param_hyb_coeff_3, param_hyb_omega_0, param_hyb_omega_2, param_hyb_omega_3, dens_threshold, zeta_threshold);
}
