//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 196/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk196(t501: f64, t171: f64, t191: f64, t187: f64, t190: f64, t177: f64) -> (f64, f64, f64, f64, f64) {
    let t531 = 0.035991666666666665_f64 * t501;
    let t533 = t171 * t191;
    let t536 = 0.006666666666666667_f64 * t190 * t533 * t187;
    let t537 = 1.0_f64 / t177;
    let t538 = t191 * t537;
    (t531, t533, t536, t537, t538)
}
