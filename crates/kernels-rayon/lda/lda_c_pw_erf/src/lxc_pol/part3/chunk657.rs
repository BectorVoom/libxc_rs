//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 657/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk657(t3953: f64, t519: f64, t1498: f64, t568: f64, t646: f64, t695: f64, t1198: f64, t1426: f64, t458: f64, t108: f64, t492: f64, t267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3955 = 4.0_f64 / 9.0_f64 * t519 * t3953;
    let t3956 = t1498 * t568;
    let t3957 = 4.0_f64 / 15.0_f64 * t3956;
    let t3959 = 0.06649088888888889_f64 * t695 * t646;
    let t3960 = t1198 * t646;
    let t3963 = 0.09973633333333333_f64 * t458 * t1426;
    let t3964 = t492 * t108;
    let t3965 = t3964 * t267;
    (t3955, t3956, t3957, t3959, t3960, t3963, t3964, t3965)
}
