//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 627/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk627(t352: f64, t93: f64, t108: f64, t2954: f64, t2961: f64, t2967: f64, t2973: f64, t3688: f64, t406: f64, t408: f64, t659: f64, t661: f64, t945: f64, t954: f64) -> f64 {
    let t3695 = t93 * t352;
    let t3701 = (40.0_f64 / 27.0_f64 * t406 * t2954 + 20.0_f64 / 3.0_f64 * t3688 * t945 + 4.0_f64 / 3.0_f64 * t659 * t2961 + 40.0_f64 / 27.0_f64 * t408 * t2967 + 20.0_f64 / 3.0_f64 * t3695 * t954 + 4.0_f64 / 3.0_f64 * t661 * t2973) * t108;
    t3701
}
