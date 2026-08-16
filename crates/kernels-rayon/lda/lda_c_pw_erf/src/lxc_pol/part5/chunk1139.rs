//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1139/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1139(t21024: f64, t1325: f64, t3859: f64, t7737: f64, t2325: f64, t4632: f64, t4829: f64, t16084: f64, t16092: f64, t4804: f64, t7738: f64, t2146: f64, t6287: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21025 = 16.0_f64 / 45.0_f64 * t21024;
    let t21027 = t1325 * t3859 * t7737;
    let t21028 = 32.0_f64 / 45.0_f64 * t21027;
    let t21032 = 16.0_f64 / 15.0_f64 * t1325 * t4829 * t4632 * t2325;
    let t21033 = 16.0_f64 / 45.0_f64 * t16084;
    let t21034 = 16.0_f64 / 45.0_f64 * t16092;
    let t21036 = 16.0_f64 / 15.0_f64 * t4804 * t7738;
    let t21038 = 4.0_f64 / 15.0_f64 * t2146 * t6287;
    (t21025, t21028, t21032, t21033, t21034, t21036, t21038)
}
