//! GGA_X_SFAT lxc pol kernel — lxc_pol (nested-by-output, 10 parts).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

use part0::gga_x_sfat_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2;
use part1::gga_x_sfat_lxc_pol_part1_v3rho3;
use part2::gga_x_sfat_lxc_pol_part2_v3rho2sigma;
use part3::gga_x_sfat_lxc_pol_part3_v3rhosigma2_v3sigma3;
use part4::gga_x_sfat_lxc_pol_part4_v4rho4;
use part5::gga_x_sfat_lxc_pol_part5_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2_v4rho3sigma_3_v4rh_etc;
use part6::gga_x_sfat_lxc_pol_part6_v4rho3sigma_6_v4rho3sigma_7_v4rho3sigma_8_v4rho3sigma_9_v4rh_etc;
use part7::gga_x_sfat_lxc_pol_part7_v4rho3sigma_11;
use part8::gga_x_sfat_lxc_pol_part8_v4rho2sigma2;
use part9::gga_x_sfat_lxc_pol_part9_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sfat_lxc_pol(
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
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_sfat_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part1_v3rho3(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part2_v3rho2sigma(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part3_v3rhosigma2_v3sigma3(rho, sigma, v3rhosigma2, v3sigma3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part4_v4rho4(rho, sigma, v4rho4, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part5_v4rho3sigma_0_v4rho3sigma_1_v4rho3sigma_2_v4rho3sigma_3_v4rh_etc(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part6_v4rho3sigma_6_v4rho3sigma_7_v4rho3sigma_8_v4rho3sigma_9_v4rh_etc(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part7_v4rho3sigma_11(rho, sigma, v4rho3sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part8_v4rho2sigma2(rho, sigma, v4rho2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_sfat_lxc_pol_part9_v4rhosigma3_v4sigma4(rho, sigma, v4rhosigma3, v4sigma4, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
