//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1137/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1137(t1447: f64, t4589: f64, t1995: f64, t3198: f64, t1444: f64, t5176: f64, t5319: f64, t3226: f64, t3284: f64, t493: f64, t6119: f64, t5180: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13507 = t1447 * t4589;
    let t13508 = 2.0_f64 / 27.0_f64 * t13507;
    let t13510 = t3198 * t1995 / 5.0_f64;
    let t13512 = 2.0_f64 / 5.0_f64 * t1444 * t5176;
    let t13514 = t1444 * t5319 / 5.0_f64;
    let t13515 = t3226 * t1995;
    let t13516 = 4.0_f64 / 15.0_f64 * t13515;
    let t13519 = t493 * t6119 * t3284 / 5.0_f64;
    let t13521 = 2.0_f64 / 5.0_f64 * t1444 * t5180;
    (t13508, t13510, t13512, t13514, t13516, t13519, t13521)
}
