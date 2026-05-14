//! GGA_X_WPBEH kxc pol kernel — kxc_pol (nested-by-output, 11 parts).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::special::{xc_erfcx};

use part0::gga_x_wpbeh_kxc_pol_part0_zk_vrho_vsigma_v2rho2;
use part1::gga_x_wpbeh_kxc_pol_part1_v2rhosigma_v2sigma2;
use part2::gga_x_wpbeh_kxc_pol_part2_v3rho3_0;
use part3::gga_x_wpbeh_kxc_pol_part3_v3rho3_1;
use part4::gga_x_wpbeh_kxc_pol_part4_v3rho3_2;
use part5::gga_x_wpbeh_kxc_pol_part5_v3rho3_3;
use part6::gga_x_wpbeh_kxc_pol_part6_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2;
use part7::gga_x_wpbeh_kxc_pol_part7_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5;
use part8::gga_x_wpbeh_kxc_pol_part8_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8;
use part9::gga_x_wpbeh_kxc_pol_part9_v3rhosigma2;
use part10::gga_x_wpbeh_kxc_pol_part10_v3sigma3;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_wpbeh_kxc_pol(
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
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_wpbeh_kxc_pol_part0_zk_vrho_vsigma_v2rho2(rho, sigma, zk, vrho, vsigma, v2rho2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part1_v2rhosigma_v2sigma2(rho, sigma, v2rhosigma, v2sigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part2_v3rho3_0(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part3_v3rho3_1(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part4_v3rho3_2(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part5_v3rho3_3(rho, sigma, v3rho3, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part6_v3rho2sigma_0_v3rho2sigma_1_v3rho2sigma_2(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part7_v3rho2sigma_3_v3rho2sigma_4_v3rho2sigma_5(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part8_v3rho2sigma_6_v3rho2sigma_7_v3rho2sigma_8(rho, sigma, v3rho2sigma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part9_v3rhosigma2(rho, sigma, v3rhosigma2, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_wpbeh_kxc_pol_part10_v3sigma3(rho, sigma, v3sigma3, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
