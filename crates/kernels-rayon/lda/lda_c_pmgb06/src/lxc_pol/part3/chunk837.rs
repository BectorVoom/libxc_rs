//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 837/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk837(t1309: f64, t374: f64, t342: f64, t384: f64, t4044: f64, t1186: f64, t1770: f64, t4243: f64, t1768: f64, t2837: f64, t398: f64, t419: f64, t4238: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8057 = t1309 * t374;
    let t8061 = t1309 * t342;
    let t8065 = t384 * t4044;
    let t8070 = t4243 * t1186 * t1770;
    let t8074 = 0.00010931146159029059_f64 * t1768 * t2837 * t1770;
    let t8077 = t4238 * t398 * t419 * t1770;
    (t8057, t8061, t8065, t8070, t8074, t8077)
}
