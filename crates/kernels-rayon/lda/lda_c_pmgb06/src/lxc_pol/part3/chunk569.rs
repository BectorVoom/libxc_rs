//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 569/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk569(t1439: f64, t3010: f64, t442: f64, t439: f64, t183: f64, t2803: f64, t1166: f64, t539: f64, t188: f64, t1830: f64, t2060: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3011 = t1439 * t3010;
    let t3012 = t442 * t3011;
    let t3014 = 2.0_f64 / 15.0_f64 * t439 * t3012;
    let t3015 = t2803 * t183;
    let t3018 = t1166 * t539;
    let t3019 = t3018 * t188;
    let t3023 = 1.2833333333333334_f64 * t1830 - 20.0_f64 / 27.0_f64 * t2060;
    let t3024 = t83 * t3023;
    (t3011, t3012, t3014, t3015, t3018, t3019, t3023, t3024)
}
