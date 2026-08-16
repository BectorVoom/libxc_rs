//! MGGA_C_B94 fxc pol kernel — fxc_pol (nested-by-output, 4 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;
mod part3;

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

use part0::mgga_c_b94_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2;
use part1::mgga_c_b94_fxc_pol_part1_v2rhosigma_v2rholapl;
use part2::mgga_c_b94_fxc_pol_part2_v2rhotau_v2sigma2_v2sigmalapl;
use part3::mgga_c_b94_fxc_pol_part3_v2sigmatau_v2lapl2_v2lapltau_v2tau2;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b94_fxc_pol(
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
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    mgga_c_b94_fxc_pol_part0_zk_vrho_vsigma_vlapl_vtau_v2rho2(rho, sigma, lapl, tau, zk, vrho, vsigma, vlapl, vtau, v2rho2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_fxc_pol_part1_v2rhosigma_v2rholapl(rho, sigma, lapl, tau, v2rhosigma, v2rholapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_fxc_pol_part2_v2rhotau_v2sigma2_v2sigmalapl(rho, sigma, lapl, tau, v2rhotau, v2sigma2, v2sigmalapl, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
    mgga_c_b94_fxc_pol_part3_v2sigmatau_v2lapl2_v2lapltau_v2tau2(rho, sigma, lapl, tau, v2sigmatau, v2lapl2, v2lapltau, v2tau2, param_cab, param_css, param_gamma, dens_threshold, zeta_threshold);
}
