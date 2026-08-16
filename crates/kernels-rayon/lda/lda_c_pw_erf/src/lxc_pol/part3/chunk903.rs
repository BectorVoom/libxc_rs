//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 903/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk903(t145: f64, t2853: f64, t164: f64, t4100: f64, t479: f64, t4120: f64, t1198: f64, t1590: f64, t4263: f64, t458: f64, t1203: f64, t1191: f64, t163: f64, t169: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9196 = t145 * t2853;
    let t9197 = t9196 * t164;
    let t9199 = t4100 * t479;
    let t9201 = t4120 * t164;
    let t9203 = t1198 * t1590;
    let t9206 = 0.12602162889256446_f64 * t458 * t4263;
    let t9207 = t1203 * t1590;
    let t9211 = t169 * t1191 * t616 * t163;
    (t9196, t9197, t9199, t9201, t9203, t9206, t9207, t9211)
}
