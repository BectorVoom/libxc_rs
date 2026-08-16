//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1164/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1164(t1455: f64, t6134: f64, t1467: f64, t1972: f64, t4585: f64, t4589: f64, t2002: f64, t5203: f64, t5198: f64, t432: f64, t6675: f64, t1180: f64, t139: f64, t30: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15304 = t6134 * t1455 / 45.0_f64;
    let t15306 = t6134 * t1467 / 27.0_f64;
    let t15308 = 2.0_f64 / 45.0_f64 * t1972 * t4585;
    let t15310 = 2.0_f64 / 27.0_f64 * t1972 * t4589;
    let t15312 = 4.0_f64 / 15.0_f64 * t2002 * t5203;
    let t15314 = 4.0_f64 / 15.0_f64 * t2002 * t5198;
    let t15316 = t432 * t6675 / 15.0_f64;
    let t15323 = t30 * t1180 * t139;
    (t15304, t15306, t15308, t15310, t15312, t15314, t15316, t15323)
}
