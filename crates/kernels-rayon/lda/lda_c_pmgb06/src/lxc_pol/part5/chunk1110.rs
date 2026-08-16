//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1110/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1110(t2492: f64, t439: f64, t4779: f64, t16743: f64, t1972: f64, t6528: f64, t6254: f64, t6550: f64, t6258: f64, t20323: f64, t20324: f64, t20325: f64, t20328: f64, t20330: f64, t20332: f64, t20334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20337 = 2.0_f64 / 15.0_f64 * t439 * t4779 * t2492;
    let t20338 = 2.0_f64 / 15.0_f64 * t16743;
    let t20340 = 2.0_f64 / 5.0_f64 * t1972 * t6528;
    let t20343 = 3.0_f64 / 5.0_f64 * t439 * t6550 * t6254;
    let t20346 = 2.0_f64 / 5.0_f64 * t439 * t6550 * t6258;
    let t20347 = t20323 + t20324 + t20325 - t20328 - t20330 - t20332 - t20334 - t20337 + t20338 + t20340 - t20343 + t20346;
    (t20337, t20338, t20340, t20343, t20346, t20347)
}
