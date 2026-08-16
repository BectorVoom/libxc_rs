//! HYB_GGA_XC_CASE21 lxc pol kernel — lxc_pol (nested-by-output, 3 parts).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod part0;
mod part1;
mod part2;

use libxc_rkernel_math::bspline::{case21_cbspline, case21_xbspline};
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use part0::hyb_gga_xc_case21_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc;
use part1::hyb_gga_xc_case21_lxc_pol_part1_v4rho4;
use part2::hyb_gga_xc_case21_lxc_pol_part2_v4rho3sigma_v4rho2sigma2_v4rhosigma3_v4sigma4;

#[allow(unused_variables, non_snake_case)]
pub fn hyb_gga_xc_case21_lxc_pol(
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
    param_ax: f64,
    param_gammac: f64,
    param_gammax: f64,
    param_cx_0: f64,
    param_cx_1: f64,
    param_cx_2: f64,
    param_cx_3: f64,
    param_cx_4: f64,
    param_cx_5: f64,
    param_cx_6: f64,
    param_cx_7: f64,
    param_cx_8: f64,
    param_cx_9: f64,
    param_cc_0: f64,
    param_cc_1: f64,
    param_cc_2: f64,
    param_cc_3: f64,
    param_cc_4: f64,
    param_cc_5: f64,
    param_cc_6: f64,
    param_cc_7: f64,
    param_cc_8: f64,
    param_cc_9: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    hyb_gga_xc_case21_lxc_pol_part0_zk_vrho_vsigma_v2rho2_v2rhosigma_v2sigma2_v3rho3_v3rho2sigma_etc(rho, sigma, zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, param_ax, param_gammac, param_gammax, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9, dens_threshold, zeta_threshold);
    hyb_gga_xc_case21_lxc_pol_part1_v4rho4(rho, sigma, v4rho4, param_ax, param_gammac, param_gammax, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9, dens_threshold, zeta_threshold);
    hyb_gga_xc_case21_lxc_pol_part2_v4rho3sigma_v4rho2sigma2_v4rhosigma3_v4sigma4(rho, sigma, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4, param_ax, param_gammac, param_gammax, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9, dens_threshold, zeta_threshold);
}
