//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 901/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk901(t405: f64, t9118: f64, t2765: f64, t2777: f64, t411: f64, t3357: f64, t1729: f64, t2763: f64, t1664: f64, t440: f64, t164: f64, t8756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9164 = t405 * t9118;
    let t9166 = t2765 * t2777 * t411;
    let t9169 = t2765 * t3357;
    let t9172 = t1729 * t2763;
    let t9174 = t2765 * t1664 * t440;
    let t9178 = 0.0014238371845981686_f64 * t8756 * t164;
    (t9164, t9166, t9169, t9172, t9174, t9178)
}
