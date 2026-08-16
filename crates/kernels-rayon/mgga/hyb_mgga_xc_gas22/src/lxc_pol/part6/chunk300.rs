//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 300/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk300(t1025: f64, t221: f64, t454: f64, t450: f64, t441: f64, t10: f64, t442: f64, t16: f64, t567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1028 = 0.11073470983333333333e-2_f64 * t221 * t1025 * t454;
    let t1029 = t450 * t450;
    let t1030 = 1.0_f64 / t1029;
    let t1031 = t441 * t1030;
    let t1033 = 1.0_f64 / t442 * t10;
    let t1034 = t16 * t567;
    let t1035 = t1033 * t1034;
    let t1037 = t221 * t1025;
    (t1028, t1029, t1030, t1031, t1033, t1034, t1035, t1037)
}
