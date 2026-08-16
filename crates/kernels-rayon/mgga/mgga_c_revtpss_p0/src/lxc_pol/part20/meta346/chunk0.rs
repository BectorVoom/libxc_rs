//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1273/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1273(t12712: f64, t3629: f64, t12702: f64, t5330: f64, t12744: f64, t1214: f64, t5341: f64, t1250: f64, t140: f64, t3698: f64, t1012: f64, t13026: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17354 = t12712 * t3629;
    let t17426 = t12702 * t5330;
    let t17429 = t12744 * t5330;
    let t17454 = t5341 * t1214;
    let t17459 = t1250 * t1214;
    let t17471 = t140 * t3698;
    let t17475 = t1012 * t13026;
    (t17354, t17426, t17429, t17454, t17459, t17471, t17475)
}
