//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 877/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk877(t10147: f64, t771: f64, t2316: f64, t3188: f64, t284: f64, t2902: f64, t3216: f64, t3218: f64, t3231: f64, t3243: f64, t2786: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10148 = t771 * t10147;
    let t10150 = t3188 * t2316;
    let t10151 = t284 * t10150;
    let t10153 = t2902 * t3216;
    let t10154 = t10153 * t3218;
    let t10156 = t3243 * t3231;
    let t10158 = t2786 * t825;
    (t10148, t10151, t10153, t10154, t10156, t10158)
}
