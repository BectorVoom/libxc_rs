//! GGA_X_HJS kxc pol kernel — kxc_pol (nested-by-output, 3 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

use part0::gga_x_hjs_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2;
use part1::gga_x_hjs_kxc_pol_part1_v3rho3;
use part2::gga_x_hjs_kxc_pol_part2_v3rho2sigma_v3rhosigma2_v3sigma3;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_hjs_kxc_pol(
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
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    gga_x_hjs_kxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_hjs_kxc_pol_part1_v3rho3(rho, sigma, v3rho3, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_hyb_omega_0, dens_threshold, zeta_threshold);
    gga_x_hjs_kxc_pol_part2_v3rho2sigma_v3rhosigma2_v3sigma3(rho, sigma, v3rho2sigma, v3rhosigma2, v3sigma3, param_a_0, param_a_1, param_a_2, param_a_3, param_a_4, param_a_5, param_b_0, param_b_1, param_b_2, param_b_3, param_b_4, param_b_5, param_b_6, param_b_7, param_b_8, param_hyb_omega_0, dens_threshold, zeta_threshold);
}
