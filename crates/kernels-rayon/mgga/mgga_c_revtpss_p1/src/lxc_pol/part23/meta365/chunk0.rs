//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1682/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1682(t15191: f64, t15197: f64, t4682: f64, t964: f64, t1626: f64, t3011: f64, t15125: f64, t11387: f64, t1609: f64, t4644: f64, t945: f64, t1614: f64, t2967: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15322 = 0.34431666666666666666e0_f64 * t15191;
    let t15324 = 0.13892666666666666667e0_f64 * t15197;
    let t15343 = t4682 * t964;
    let t15350 = t1626 * t3011;
    let t15363 = 0.2283111111111111111e-1_f64 * t15125;
    let t15364 = 0.11415555555555555555e-1_f64 * t15191;
    let t15396 = t1609 * t11387;
    let t15400 = t4644 * t945;
    let t15406 = t1614 * t2967;
    (t15322, t15324, t15343, t15350, t15363, t15364, t15396, t15400, t15406)
}
