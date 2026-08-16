//! MGGA_C_REVTPSS fxc pol kernel — fxc_pol (nested-by-output, 3 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use part0::mgga_c_revtpss_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau;
use part1::mgga_c_revtpss_fxc_pol_part1_v2rho2;
use part2::mgga_c_revtpss_fxc_pol_part2_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_revtpss_fxc_pol(
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
    param_C0_c_0: f64,
    param_C0_c_1: f64,
    param_C0_c_2: f64,
    param_C0_c_3: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_revtpss_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_d, dens_threshold, zeta_threshold);
    mgga_c_revtpss_fxc_pol_part1_v2rho2(rho, sigma, lapl, tau, v2rho2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_d, dens_threshold, zeta_threshold);
    mgga_c_revtpss_fxc_pol_part2_v2rhosigma_v2rholapl_v2rhotau_v2sigma2_v2sigmalapl_v2sigmata_etc(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_C0_c_0, param_C0_c_1, param_C0_c_2, param_C0_c_3, param_d, dens_threshold, zeta_threshold);
}
