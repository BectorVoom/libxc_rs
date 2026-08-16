//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 902/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk902(t4137: f64, t479: f64, t164: f64, t8832: f64, t4107: f64, t163: f64, t1645: f64, t169: f64, t717: f64, t1159: f64, t1590: f64, t695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9180 = 0.0004746123948660562_f64 * t4137 * t479;
    let t9181 = t8832 * t164;
    let t9186 = t4107 * t479;
    let t9190 = t169 * t717 * t1645 * t163;
    let t9192 = t1159 * t479;
    let t9195 = 0.3780648866776934_f64 * t695 * t1590;
    (t9180, t9181, t9186, t9190, t9192, t9195)
}
