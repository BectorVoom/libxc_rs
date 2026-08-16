//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1011/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1011(t190: f64, t5121: f64, t9821: f64, t191: f64, t21: f64, t24: f64, t1267: f64, t3476: f64, t348: f64, t739: f64, t945: f64) -> (f64, f64, f64, f64) {
    let t11851 = t190 * t9821 * t5121;
    let t11854 = t21 * t24 * t191;
    let t11855 = t1267 * t3476;
    let t11857 = t739 * t945 * t348;
    (t11851, t11854, t11855, t11857)
}
