//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1011/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1011(t12022: f64, t439: f64, t4663: f64, t5225: f64, t2002: f64, t2966: f64, t1594: f64, t1868: f64, t2010: f64, t2864: f64, t12000: f64, t12003: f64, t12005: f64, t12011: f64, t12015: f64, t12017: f64, t12019: f64, t12021: f64) -> (f64, f64, f64, f64, f64) {
    let t12023 = 4.0_f64 / 45.0_f64 * t12022;
    let t12026 = 2.0_f64 / 5.0_f64 * t439 * t5225 * t4663;
    let t12028 = 2.0_f64 / 15.0_f64 * t2002 * t2966;
    let t12032 = 4.0_f64 / 15.0_f64 * t2010 * t2864 * t1868 * t1594;
    let t12033 = -t12000 + t12003 - t12005 - t12011 - t12015 - t12017 - t12019 - t12021 + t12023 + t12026 + t12028 + t12032;
    (t12023, t12026, t12028, t12032, t12033)
}
