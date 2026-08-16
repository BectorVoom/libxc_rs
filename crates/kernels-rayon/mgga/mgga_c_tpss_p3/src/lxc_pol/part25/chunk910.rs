//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 910/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk910(t2912: f64, t407: f64, t1019: f64, t2910: f64, t2861: f64, t3153: f64, t475: f64, t126: f64, t3096: f64, t215: f64, t442: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9495 = 1.0_f64 / t2912 / t407;
    let t9504 = t1019 * t2910;
    let t9507 = t1019 * t2861;
    let t9519 = 1.0_f64 / t3153 / t475;
    let t9523 = t126 * t3096;
    let t9533 = t215 * t68 * t442;
    (t9495, t9504, t9507, t9519, t9523, t9533)
}
