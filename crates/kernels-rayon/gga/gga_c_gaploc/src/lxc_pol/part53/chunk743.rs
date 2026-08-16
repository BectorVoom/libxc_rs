//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 743/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk743(t1535: f64, t9419: f64, t6519: f64, t9439: f64, t9448: f64, t10531: f64, t1433: f64, t1065: f64, t883: f64, t900: f64, t1423: f64, t6589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20687 = t1535 * t9419;
    let t20696 = t9439 * t6519;
    let t20700 = t9448 * t6519;
    let t20796 = t1433 * t10531;
    let t20883 = t883 * t1065;
    let t20884 = t900 * t20883;
    let t20967 = t1423 * t6589;
    (t20687, t20696, t20700, t20796, t20883, t20884, t20967)
}
