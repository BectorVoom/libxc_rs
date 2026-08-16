//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 731/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk731(t1971: f64, t945: f64, t503: f64, t11: f64, t188: f64, t504: f64, t174: f64, t3540: f64, t3493: f64, t3530: f64, t3532: f64, t3534: f64, t3997: f64, t4600: f64, t4602: f64, t4605: f64, t4607: f64, t4612: f64, t4617: f64, t4622: f64, t4626: f64, t4630: f64, t4635: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4637 = t1971 * t945;
    let t4638 = t503 * t4637;
    let t4639 = t11 * t4638;
    let t4641 = t188 * t504;
    let t4643 = t174 * t3540 * t4641;
    let t4645 = t3997 + 0.0016792592592592592_f64 * t3530 - 0.0004198148148148148_f64 * t3534 + 0.0012594444444444445_f64 * t3493 - 0.0006297222222222223_f64 * t3532 + 0.0008396296296296296_f64 * t4600 - 0.0008396296296296296_f64 * t4602 + t4605 - 0.01385388888888889_f64 * t4607 + 0.002099074074074074_f64 * t4612 - 0.007556666666666666_f64 * t4617 + 0.005037777777777778_f64 * t4622 + 0.0012594444444444445_f64 * t4626 + 0.011335_f64 * t4630 - 0.015113333333333333_f64 * t4635 - 0.003778333333333333_f64 * t4639 + 0.003778333333333333_f64 * t4643;
    (t4637, t4638, t4639, t4641, t4643, t4645)
}
