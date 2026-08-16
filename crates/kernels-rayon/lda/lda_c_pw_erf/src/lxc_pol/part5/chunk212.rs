//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 212/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk212(t555: f64, t190: f64, t212: f64, t533: f64, t205: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t583 = 0.035991666666666665_f64 * t555;
    let t587 = 0.006666666666666667_f64 * t190 * t533 * t212;
    let t588 = 1.0_f64 / t205;
    let t589 = t191 * t588;
    (t583, t587, t588, t589)
}
