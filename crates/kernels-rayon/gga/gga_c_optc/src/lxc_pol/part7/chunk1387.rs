//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1387/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1387(t1162: f64, t3088: f64, t7274: f64, t3097: f64, t1179: f64, t27004: f64, t8470: f64, t9170: f64, t3181: f64, t442: f64, t462: f64, t27173: f64) -> (f64, f64, f64, f64, f64) {
    let t27616 = t1162 * t7274 * t3088;
    let t27619 = t1162 * t7274 * t3097;
    let t27621 = t1179 * t27004;
    let t27623 = t9170 * t8470;
    let t27629 = 1.0_f64 / t3181 / t462 * t442;
    let t27630 = t27629 * t27173;
    (t27616, t27619, t27621, t27623, t27630)
}
