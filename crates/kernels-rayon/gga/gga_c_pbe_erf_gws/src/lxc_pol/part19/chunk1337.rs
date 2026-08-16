//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1337/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1337(t14469: f64, t53229: f64, t11422: f64, t13917: f64, t53447: f64, t15342: f64, t53891: f64, t11651: f64, t13776: f64, t52906: f64, t14657: f64, t54595: f64) -> (f64, f64, f64, f64, f64) {
    let t57488 = t53229 * t14469;
    let t57495 = t13917 * t53447 * t11422;
    let t57497 = t53891 * t15342;
    let t57500 = t13776 * t52906 * t11651;
    let t57506 = t14657 * t54595;
    (t57488, t57495, t57497, t57500, t57506)
}
