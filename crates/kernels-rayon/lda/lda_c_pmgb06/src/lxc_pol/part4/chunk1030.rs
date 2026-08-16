//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1030/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1030(t1190: f64, t4189: f64, t1187: f64, t4197: f64, t115: f64, t8173: f64, t247: f64, t413: f64, t113: f64, t642: f64, t8193: f64, t1321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10520 = 0.2508_f64 * t4189 * t1190;
    let t10522 = 0.39013333333333333_f64 * t1187 * t4197;
    let t10524 = t8173 * t115;
    let t10525 = t10524 / 2.0_f64;
    let t10528 = 0.007532237109403992_f64 * t413 * t247 * t115;
    let t10531 = 0.015064474218807983_f64 * t113 * t642 * t115;
    let t10533 = 60.0_f64 * t8193;
    let t10547 = t1321 * t1321;
    (t10520, t10522, t10524, t10525, t10528, t10531, t10533, t10547)
}
