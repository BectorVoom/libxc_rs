//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 888/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk888(t1073: f64, t3007: f64, t1184: f64, t119: f64, t395: f64, t84: f64, t174: f64, t473: f64, t903: f64, t908: f64, t912: f64, t914: f64) -> (f64, f64, f64, f64) {
    let t8464 = t1073 * t3007;
    let t8469 = 0.0018989760778855128_f64 * t395 * t119 * t1184 * t84;
    let t8473 = 2.291123905095794_f64 * t174 * t473 * t903 * t908;
    let t8477 = 0.2849333333333333_f64 * t174 * t473 * t912 * t914;
    (t8464, t8469, t8473, t8477)
}
