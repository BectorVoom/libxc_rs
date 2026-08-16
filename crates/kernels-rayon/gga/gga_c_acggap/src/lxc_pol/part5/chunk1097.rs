//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1097/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1097(t1938: f64, t3892: f64, t1907: f64, t310: f64, t464: f64, t1219: f64, t1937: f64, t5384: f64, t871: f64, t6438: f64, t857: f64, t6558: f64) -> (f64, f64, f64, f64, f64) {
    let t19664 = t3892 * t1938;
    let t19667 = t310 * t1907;
    let t19668 = t19667 * t464;
    let t19672 = t5384 * t1219 * t1937 * t871;
    let t19676 = t857 * t6438;
    let t19678 = t857 * t6558;
    (t19664, t19668, t19672, t19676, t19678)
}
