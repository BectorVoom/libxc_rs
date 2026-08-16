//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1242/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1242(t16354: f64, t1919: f64, t493: f64, t1080: f64, t2386: f64, t9525: f64, t5470: f64, t1444: f64, t6509: f64, t12580: f64, t6508: f64, t16321: f64, t16336: f64, t16338: f64, t16339: f64, t16340: f64, t16342: f64, t16345: f64, t16347: f64, t16349: f64, t16351: f64, t16353: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16357 = 2.0_f64 / 9.0_f64 * t493 * t1919 * t16354;
    let t16359 = t9525 * t2386 * t1080;
    let t16362 = 32.0_f64 / 27.0_f64 * t493 * t5470 * t16359;
    let t16364 = 16.0_f64 / 81.0_f64 * t1444 * t6509;
    let t16367 = 16.0_f64 / 81.0_f64 * t493 * t12580 * t6508;
    let t16368 = -t16321 + t16336 + t16338 - t16339 - t16340 + t16342 + t16345 + t16347 + t16349 + t16351 + t16353 - t16357 - t16362 + t16364 + t16367;
    (t16357, t16359, t16362, t16364, t16367, t16368)
}
