//! GGA_C_OP_B88 lxc pol kernel — lxc_pol (nested-by-output, 3 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

use part0::gga_c_op_b88_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc;
use part1::gga_c_op_b88_lxc_pol_part1_v4rho3sigma_v4rho2sigma2;
use part2::gga_c_op_b88_lxc_pol_part2_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_b88_lxc_pol(
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
    gga_c_op_b88_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, v4rho4, dens_threshold, zeta_threshold);
    gga_c_op_b88_lxc_pol_part1_v4rho3sigma_v4rho2sigma2(rho, sigma, v4rho3sigma, v4rho2sigma2, dens_threshold, zeta_threshold);
    gga_c_op_b88_lxc_pol_part2_v4rhosigma3_v4sigma4(rho, sigma, v4rhosigma3, v4sigma4, dens_threshold, zeta_threshold);
}
