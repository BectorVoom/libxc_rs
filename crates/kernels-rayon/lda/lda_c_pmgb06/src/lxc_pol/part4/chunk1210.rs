//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1210/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1210(t1381: f64, t15947: f64, t493: f64, t439: f64, t5232: f64, t5482: f64, t2497: f64, t3198: f64, t1444: f64, t6387: f64, t6391: f64, t15923: f64, t15925: f64, t15927: f64, t15930: f64, t15934: f64, t15939: f64, t15942: f64, t15944: f64, t15946: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15950 = 2.0_f64 / 45.0_f64 * t493 * t15947 * t1381;
    let t15953 = 4.0_f64 / 45.0_f64 * t439 * t5482 * t5232;
    let t15955 = 2.0_f64 / 45.0_f64 * t3198 * t2497;
    let t15957 = 4.0_f64 / 45.0_f64 * t1444 * t6387;
    let t15959 = 4.0_f64 / 45.0_f64 * t1444 * t6391;
    let t15960 = -t15923 - t15925 - t15927 - t15930 - t15934 - t15939 + t15942 - t15944 - t15946 - t15950 - t15953 - t15955 - t15957 - t15959;
    (t15950, t15953, t15955, t15957, t15959, t15960)
}
