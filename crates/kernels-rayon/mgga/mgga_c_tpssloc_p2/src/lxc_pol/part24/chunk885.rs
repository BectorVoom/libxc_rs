//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 885/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk885(t2710: f64, t814: f64, t829: f64, t252: f64, t9971: f64, t9976: f64, t2728: f64, t9981: f64, t2684: f64, t2732: f64, t6647: f64, t9632: f64) -> (f64, f64, f64, f64, f64) {
    let t10076 = t814 * t2710;
    let t10077 = t10076 * t829;
    let t10080 = t9971 * t252;
    let t10081 = t10080 * t9976;
    let t10084 = t2728 * t9981;
    let t10091 = t2732 * t2684;
    let t10094 = t6647 * t9632;
    (t10077, t10081, t10084, t10091, t10094)
}
