//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1323/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1323(t17188: f64, t1919: f64, t493: f64, t1444: f64, t6766: f64, t5463: f64, t6765: f64, t17143: f64, t17147: f64, t17152: f64, t5470: f64, t1464: f64, t2093: f64, t5071: f64, t5138: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17384 = 2.0_f64 / 9.0_f64 * t493 * t1919 * t17188;
    let t17386 = 2.0_f64 / 27.0_f64 * t1444 * t6766;
    let t17389 = 2.0_f64 / 27.0_f64 * t493 * t5463 * t6765;
    let t17392 = 2.0_f64 / 27.0_f64 * t493 * t1919 * t17143;
    let t17395 = t493 * t1919 * t17147 / 27.0_f64;
    let t17398 = 8.0_f64 / 81.0_f64 * t493 * t5470 * t17152;
    let t17402 = 4.0_f64 / 27.0_f64 * t5138 * t2093 * t1464 * t5071;
    (t17384, t17386, t17389, t17392, t17395, t17398, t17402)
}
