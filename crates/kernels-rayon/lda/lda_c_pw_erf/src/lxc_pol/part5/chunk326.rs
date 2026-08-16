//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 326/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk326(t1191: f64, t169: f64, t274: f64, t301: f64, t242: f64, t695: f64, t145: f64, t465: f64) -> (f64, f64, f64) {
    let t1195 = 0.19816831758676853_f64 * t169 * t1191 * t274 * t301;
    let t1197 = 0.1675256410710088_f64 * t695 * t242;
    let t1198 = t145 * t465;
    (t1195, t1197, t1198)
}
