//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 677/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk677(t6011: f64, t85: f64, t411: f64, t770: f64, t2765: f64, t1734: f64, t2591: f64, t1729: f64, t454: f64, t776: f64, t2363: f64, t299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6012 = t6011 * t85;
    let t6013 = 0.019751789702565206_f64 * t6012;
    let t6015 = t770 * t411;
    let t6016 = t2765 * t6015;
    let t6019 = t2591 * t1734;
    let t6025 = t1729 * t776 * t454;
    let t6035 = t299 * t2363;
    (t6012, t6013, t6015, t6016, t6019, t6025, t6035)
}
