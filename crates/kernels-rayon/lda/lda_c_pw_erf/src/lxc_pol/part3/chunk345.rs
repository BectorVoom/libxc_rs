//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 345/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk345(t1217: f64, t265: f64, t665: f64, t668: f64, t108: f64, t659: f64, t661: f64, t92: f64, t93: f64, t940: f64, t945: f64, t951: f64, t954: f64) -> (f64, f64, f64) {
    let t1219 = 2.0_f64 / 135.0_f64 * t265 * t1217;
    let t1220 = t665 * t668;
    let t1231 = (20.0_f64 / 9.0_f64 * t92 * t940 + 4.0_f64 / 3.0_f64 * t659 * t945 + 20.0_f64 / 9.0_f64 * t93 * t951 + 4.0_f64 / 3.0_f64 * t661 * t954) * t108;
    (t1219, t1220, t1231)
}
