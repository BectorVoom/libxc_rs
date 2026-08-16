//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 940/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk940(t10527: f64, t4051: f64, t571: f64, t1318: f64, t3899: f64, t4043: f64, t1476: f64, t3727: f64, t3892: f64, t9: f64, t3895: f64, t519: f64) -> (f64, f64, f64, f64, f64) {
    let t10529 = t571 * t10527 * t4051;
    let t10541 = t1318 * t3899 * t4043;
    let t10551 = t3727 * t1476;
    let t10557 = t9 * t3892;
    let t10559 = t519 * t10557 * t3895;
    (t10529, t10541, t10551, t10557, t10559)
}
