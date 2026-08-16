//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 585/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk585(t261: f64, t52: f64, t1563: f64, t352: f64, t2954: f64, t2961: f64, t2967: f64, t2973: f64, t3234: f64, t3237: f64, t406: f64, t408: f64, t945: f64, t954: f64) -> (f64, f64) {
    let t3243 = 1.0_f64 / t52 / t261;
    let t3246 = t1563 * t352;
    let t3251 = 4.0_f64 / 27.0_f64 * t3234 * t2954 - t3237 * t945 / 3.0_f64 + t406 * t2961 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t3243 * t2967 - t3246 * t954 / 3.0_f64 + t408 * t2973 / 3.0_f64;
    (t3243, t3251)
}
