//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 850/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk850(t4459: f64, t5727: f64, t5743: f64, t5942: f64, t312: f64, t19: f64, t2316: f64, t729: f64, t734: f64, t1729: f64, t454: f64, t776: f64) -> (f64, f64, f64, f64, f64) {
    let t5944 = t4459 + t5727 + t5743 + t5942;
    let t5945 = t5944 * t312;
    let t5949 = t2316 * t729 * t19;
    let t5950 = t5949 * t734;
    let t6025 = t1729 * t776 * t454;
    (t5944, t5945, t5949, t5950, t6025)
}
