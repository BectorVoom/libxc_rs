//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 908/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk908(t196: f64, t3674: f64, t218: f64, t3666: f64, t3437: f64, t565: f64, t198: f64, t4567: f64, t185: f64, t4062: f64, t581: f64, t3667: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9223 = 1.0_f64 / t3674 / t196;
    let t9237 = 1.0_f64 / t3666 / t218;
    let t9246 = t565 * t3437;
    let t9248 = t4567 * t198;
    let t9250 = 112.0_f64 / 1215.0_f64 * t185 * t9248;
    let t9278 = t4062 * t581;
    let t9286 = t574 * t3667;
    (t9223, t9237, t9246, t9250, t9278, t9286)
}
