//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1774/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774(t120: f64, t4119: f64, t2645: f64, t829: f64, t2679: f64, t4248: f64, t13242: f64, t4180: f64, t4181: f64, t4240: f64, t9638: f64, t2647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13300 = t120 * t4119;
    let t13302 = t2645 * t13300 * t829;
    let t13306 = t2645 * t4248 * t2679;
    let t13312 = t4180 * t13242 * t829;
    let t13316 = t4180 * t4181 * t2679;
    let t13320 = 7.0_f64 / 2304.0_f64 * t9638 * t4240;
    let t13322 = t2645 * t13242 * t2647;
    (t13302, t13306, t13312, t13316, t13320, t13322)
}
