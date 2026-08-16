//! GGA_C_PBE_ERF_GWS kxc pol kernel — kxc_pol (nested-by-output, 7 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::gga_c_pbe_erf_gws_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2;
use part1::gga_c_pbe_erf_gws_kxc_pol_part1_v3rho3_0;
use part2::gga_c_pbe_erf_gws_kxc_pol_part2_v3rho3_1;
use part3::gga_c_pbe_erf_gws_kxc_pol_part3_v3rho3_2;
use part4::gga_c_pbe_erf_gws_kxc_pol_part4_v3rho3_3;
use part5::gga_c_pbe_erf_gws_kxc_pol_part5_v3rho2sigma;
use part6::gga_c_pbe_erf_gws_kxc_pol_part6_v3rhosigma2_v3sigma3;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_pbe_erf_gws_kxc_pol(
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
    param_a_c: f64,
    param_beta: f64,
    param_gamma: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_c_pbe_erf_gws_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part1_v3rho3_0(rho, sigma, v3rho3, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part2_v3rho3_1(rho, sigma, v3rho3, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part3_v3rho3_2(rho, sigma, v3rho3, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part4_v3rho3_3(rho, sigma, v3rho3, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part5_v3rho2sigma(rho, sigma, v3rho2sigma, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_c_pbe_erf_gws_kxc_pol_part6_v3rhosigma2_v3sigma3(rho, sigma, v3rhosigma2, v3sigma3, param_a_c, param_beta, param_gamma, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
