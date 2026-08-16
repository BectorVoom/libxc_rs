//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1235/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1235(t1056: f64, t3305: f64, t2213: f64, t238: f64, t3344: f64, t3348: f64, t1342: f64, t6611: f64, t801: f64, t8693: f64, t8697: f64, t8701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24587 = 8.0_f64 * t3305 * t1056;
    let t24658 = t238 * t2213 * t3344;
    let t24661 = t238 * t2213 * t3348;
    let t24664 = t238 * t6611 * t1342;
    let t24667 = t238 * t801 * t8693;
    let t24670 = t238 * t801 * t8697;
    let t24673 = t238 * t801 * t8701;
    (t24587, t24658, t24661, t24664, t24667, t24670, t24673)
}
