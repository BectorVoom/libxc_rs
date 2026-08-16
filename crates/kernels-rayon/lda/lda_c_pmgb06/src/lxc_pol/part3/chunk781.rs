//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 781/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk781(t1423: f64, t1908: f64, t1069: f64, t1531: f64, t822: f64, t1385: f64, t439: f64, t1898: f64, t1897: f64, t4663: f64, t1902: f64, t1447: f64, t1925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5342 = 4.0_f64 / 135.0_f64 * t1423 * t1908;
    let t5344 = t822 * t1531 * t1069;
    let t5345 = t1385 * t5344;
    let t5347 = 2.0_f64 / 45.0_f64 * t439 * t5345;
    let t5349 = 8.0_f64 / 135.0_f64 * t1423 * t1898;
    let t5350 = t1897 * t4663;
    let t5352 = 2.0_f64 / 15.0_f64 * t439 * t5350;
    let t5354 = 4.0_f64 / 81.0_f64 * t1423 * t1902;
    let t5356 = 4.0_f64 / 135.0_f64 * t1447 * t1925;
    (t5342, t5344, t5345, t5347, t5349, t5350, t5352, t5354, t5356)
}
