//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3125/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3125(t3057: f64, t4930: f64, t15886: f64, t378: f64, t3046: f64, t1072: f64, t1647: f64, t3259: f64, t1071: f64, t15669: f64, t15654: f64, t12050: f64, t15907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55413 = t3057 * t4930;
    let t55416 = t15886 * t378;
    let t55421 = t3046 * t4930;
    let t55458 = t3057 * t1072;
    let t55461 = t1647 * t3259;
    let t55464 = t15669 * t1071;
    let t55475 = t15654 * t1071;
    let t55499 = t15907 * t12050;
    (t55413, t55416, t55421, t55458, t55461, t55464, t55475, t55499)
}
