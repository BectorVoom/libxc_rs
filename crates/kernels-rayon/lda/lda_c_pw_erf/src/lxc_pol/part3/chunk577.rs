//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 577/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk577(t38: f64, t461: f64, t36: f64, t88: f64, t1067: f64, t391: f64, t358: f64, t1070: f64, t1064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3165 = 1.0_f64 / t38 / t461;
    let t3166 = t36 * t3165;
    let t3167 = t3166 * t88;
    let t3168 = 120.0_f64 * t3167;
    let t3169 = t1067 * t391;
    let t3170 = 36.0_f64 * t3169;
    let t3171 = t1067 * t358;
    let t3172 = 36.0_f64 * t3171;
    let t3173 = t1070 * t391;
    let t3174 = 96.0_f64 * t3173;
    let t3175 = t1064 * t391;
    let t3176 = 60.0_f64 * t3175;
    (t3165, t3166, t3167, t3168, t3169, t3170, t3171, t3172, t3173, t3174, t3175, t3176)
}
