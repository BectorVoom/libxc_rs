//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1052/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1052(t12084: f64, t12105: f64, t12126: f64, t12147: f64, t576: f64, t3848: f64, t699: f64, t1096: f64, t11043: f64, t3828: f64, t883: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12149 = t12084 + t12105 + t12126 + t12147;
    let t12150 = t576 * t12149;
    let t12151 = t699 * t3848;
    let t12152 = t11043 * t1096;
    let t12153 = t3828 * t883;
    let t12154 = t12153 * t972;
    (t12149, t12150, t12151, t12152, t12153, t12154)
}
