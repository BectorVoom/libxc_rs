//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 224/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk224(t168: f64, t270: f64, t635: f64, t247: f64, t465: f64, t251: f64, t147: f64, t19: f64, t3: f64) -> (f64, f64, f64, f64, f64) {
    let t638 = 0.019897291109174608_f64 * t168 * t635 * t270;
    let t639 = t465 * t247;
    let t640 = t639 * t251;
    let t643 = t147 * t19;
    let t644 = 1.0_f64 / t3;
    (t638, t639, t640, t643, t644)
}
