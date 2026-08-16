//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1245/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1245(t14046: f64, t3172: f64, t14565: f64, t346: f64, t838: f64, t859: f64, t4142: f64, t51529: f64, t13953: f64, t14648: f64, t51877: f64, t13972: f64, t14684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54397 = t14046 * t3172;
    let t54401 = t14565 * t346 * t838 * t859;
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54435 = 35.0_f64 / 216.0_f64 * t51877;
    let t54463 = t13972 * t14684;
    (t54397, t54401, t54427, t54429, t54435, t54463)
}
