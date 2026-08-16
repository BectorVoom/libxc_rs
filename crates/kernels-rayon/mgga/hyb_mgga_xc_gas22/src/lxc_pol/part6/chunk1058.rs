//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1058/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1058(t10080: f64, t29: f64, t125: f64, t26: f64, t10: f64, t18: f64, t9909: f64, t3806: f64, t551: f64, t127: f64, t19: f64, t3919: f64, t547: f64, t5876: f64, t5880: f64, t642: f64, t670: f64, t7879: f64, t7881: f64, t7885: f64, t7887: f64) -> (f64, f64, f64, f64, f64) {
    let t10081 = t29 * t10080;
    let t10082 = t10081 * t125;
    let t10083 = t26 * t10082;
    let t10087 = t9909 * t10 * t18;
    let t10094 = t3806 * t551;
    let t10097 = -3.0_f64 / 64.0_f64 * t547 * t3919 - t7879 + t7881 / 48.0_f64 - t7885 / 16.0_f64 + t7887 / 48.0_f64 - 3.0_f64 / 64.0_f64 * t19 * t10083 - 3.0_f64 / 64.0_f64 * t10087 * t127 - 3.0_f64 / 64.0_f64 * t3806 * t642 - 3.0_f64 / 64.0_f64 * t3806 * t670 - t10094 / 64.0_f64 + t5876 / 96.0_f64 - t5880;
    (t10081, t10082, t10083, t10087, t10097)
}
