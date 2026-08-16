//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1213/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1213(t1420: f64, t6365: f64, t1385: f64, t439: f64, t5039: f64, t809: f64, t15962: f64, t15965: f64, t15968: f64, t15971: f64, t15973: f64, t15975: f64, t15978: f64, t15980: f64, t15982: f64, t15983: f64, t15984: f64, t15987: f64, t15990: f64) -> (f64, f64, f64) {
    let t15992 = 4.0_f64 / 45.0_f64 * t1420 * t6365;
    let t15996 = 2.0_f64 / 45.0_f64 * t439 * t1385 * t809 * t5039;
    let t15997 = t15962 - t15965 - t15968 + t15971 - t15973 + t15975 + t15978 + t15980 - t15982 - t15983 - t15984 - t15987 - t15990 - t15992 - t15996;
    (t15992, t15996, t15997)
}
