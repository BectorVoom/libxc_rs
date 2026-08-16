//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 793/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk793(t1983: f64, t5499: f64, t1382: f64, t1444: f64, t1920: f64, t1925: f64, t1972: f64, t1981: f64, t3451: f64, t3454: f64, t439: f64, t493: f64, t5458: f64, t5464: f64, t5467: f64, t5471: f64, t5474: f64, t5477: f64, t5483: f64, t5487: f64, t5494: f64, t5497: f64) -> f64 {
    let t5500 = t5499 * t1983;
    let t5504 = 8.0_f64 / 45.0_f64 * t1981 * t5458 + 2.0_f64 / 27.0_f64 * t1444 * t1920 + 2.0_f64 / 27.0_f64 * t493 * t5464 + t493 * t5467 / 27.0_f64 + 8.0_f64 / 81.0_f64 * t493 * t5471 - 4.0_f64 / 27.0_f64 * t1981 * t5474 - 2.0_f64 / 45.0_f64 * t493 * t5477 - 2.0_f64 / 45.0_f64 * t1972 * t1382 - 2.0_f64 / 45.0_f64 * t439 * t5483 - 2.0_f64 / 45.0_f64 * t493 * t5487 - 2.0_f64 / 45.0_f64 * t1444 * t1925 - 2.0_f64 / 45.0_f64 * t493 * t5494 - 2.0_f64 / 405.0_f64 * t5497 + 2.0_f64 / 27.0_f64 * t5500 + 2.0_f64 / 135.0_f64 * t3451 - t3454 / 45.0_f64;
    t5504
}
