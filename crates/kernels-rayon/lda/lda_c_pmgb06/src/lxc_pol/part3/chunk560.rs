//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 560/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk560(t2938: f64, t497: f64, t506: f64, t36: f64, t2900: f64, t2901: f64, t2903: f64, t2905: f64, t2907: f64, t2915: f64, t2921: f64, t2926: f64, t2930: f64, t2935: f64) -> (f64, f64, f64, f64) {
    let t2939 = t497 * t2938;
    let t2940 = t506 * t2939;
    let t2941 = t36 * t2940;
    let t2943 = t2900 + 0.002518888888888889_f64 * t2901 - 0.0012594444444444445_f64 * t2903 + 0.003778333333333333_f64 * t2905 - 0.0018891666666666666_f64 * t2907 + 0.002099074074074074_f64 * t2915 - 0.007556666666666666_f64 * t2921 + 0.003778333333333333_f64 * t2926 + 0.011335_f64 * t2930 - 0.011335_f64 * t2935 + 0.0018891666666666666_f64 * t2941;
    (t2939, t2940, t2941, t2943)
}
