//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 313/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk313(t1084: f64, t1085: f64, t169: f64, t301: f64, t678: f64, t717: f64, t147: f64) -> (f64, f64, f64) {
    let t1086 = t1084 * t1085;
    let t1087 = 0.010843580882781523_f64 * t1086;
    let t1096 = t169 * t717 * t678 * t301;
    let t1098 = t717 * t147;
    (t1087, t1096, t1098)
}
