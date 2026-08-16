//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1065/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1065(t1444: f64, t4762: f64, t1989: f64, t3223: f64, t4761: f64, t493: f64, t5179: f64, t9596: f64, t9598: f64, t9601: f64, t1980: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12648 = 3.0_f64 / 5.0_f64 * t1444 * t4762;
    let t12649 = t3223 * t1989;
    let t12650 = 2.0_f64 / 135.0_f64 * t12649;
    let t12653 = 3.0_f64 / 5.0_f64 * t493 * t5179 * t4761;
    let t12654 = 4.0_f64 / 135.0_f64 * t9596;
    let t12655 = 2.0_f64 / 45.0_f64 * t9598;
    let t12656 = 2.0_f64 / 45.0_f64 * t9601;
    let t12657 = t883 * t1980;
    (t12648, t12650, t12653, t12654, t12655, t12656, t12657)
}
