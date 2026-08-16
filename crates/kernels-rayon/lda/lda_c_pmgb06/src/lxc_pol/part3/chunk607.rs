//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 607/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk607(t2899: f64, t2901: f64, t2903: f64, t2905: f64, t2907: f64, t2915: f64, t2921: f64, t2926: f64, t2930: f64, t2935: f64, t2941: f64, t3369: f64) -> (f64, f64) {
    let t3380 = 0.11197407407407407_f64 * t2899;
    let t3381 = -0.21595_f64 * t2930 + 0.21595_f64 * t2935 - 0.07198333333333333_f64 * t2905 + 0.14396666666666666_f64 * t2921 - 0.07198333333333333_f64 * t2926 - 0.047988888888888886_f64 * t2901 + 0.035991666666666665_f64 * t2907 + 0.023994444444444443_f64 * t2903 - 0.03999074074074074_f64 * t2915 - 0.035991666666666665_f64 * t2941 - t3380;
    let t3382 = t3369 + t3381;
    (t3380, t3382)
}
