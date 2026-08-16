//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 675/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk675(t1867: f64, t6407: f64, t3649: f64, t3696: f64, t6364: f64, t6367: f64, t6370: f64, t6375: f64, t6377: f64, t6379: f64, t587: f64, t1863: f64, t579: f64) -> (f64, f64, f64, f64) {
    let t6408 = t6407 * t1867;
    let t6419 = -0.34523333333333333333e1_f64 * t6364 + 0.23015555555555555556e1_f64 * t6367 - 0.26851481481481481482e1_f64 * t6370 - 0.93932222222222222223e0_f64 * t3649 + 0.73355e-1_f64 * t6375 - 0.14671e0_f64 * t6377 - 0.17116166666666666667e0_f64 * t6379 - 0.36793333333333333333e0_f64 * t3696;
    let t6420 = t6419 * t587;
    let t6424 = 1.0_f64 / t1863 / t579;
    (t6408, t6419, t6420, t6424)
}
