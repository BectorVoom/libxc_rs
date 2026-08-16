//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2449/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449(t3046: f64, t3298: f64, t4891: f64, t11263: f64, t3169: f64, t11977: f64, t3173: f64, t12009: f64, t12013: f64, t11916: f64, t11999: f64, t11874: f64, t16048: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42643 = t3046 * t3298 * t4891;
    let t42656 = t3169 * t11263;
    let t42658 = t11977 * t3173;
    let t42660 = t12013 * t12009;
    let t42662 = t11999 * t11916;
    let t42675 = t11874 * t16048;
    (t42643, t42656, t42658, t42660, t42662, t42675)
}
