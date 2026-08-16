//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 907/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk907(t164: f64, t8756: f64, t4137: f64, t479: f64, t8832: f64, t1159: f64, t1590: f64, t695: f64, t1198: f64, t4263: f64, t458: f64, t1191: f64, t163: f64, t169: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9178 = 0.0014238371845981686_f64 * t8756 * t164;
    let t9180 = 0.0004746123948660562_f64 * t4137 * t479;
    let t9181 = t8832 * t164;
    let t9192 = t1159 * t479;
    let t9195 = 0.3780648866776934_f64 * t695 * t1590;
    let t9203 = t1198 * t1590;
    let t9206 = 0.12602162889256446_f64 * t458 * t4263;
    let t9211 = t169 * t1191 * t616 * t163;
    (t9178, t9180, t9181, t9192, t9195, t9203, t9206, t9211)
}
