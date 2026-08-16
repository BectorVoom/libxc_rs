//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1084/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1084(t1289: f64, t5211: f64, t2076: f64, t3565: f64, t3660: f64, t1325: f64, t4632: f64, t4829: f64, t940: f64, t1997: f64, t3745: f64, t3859: f64, t5413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12681 = 4.0_f64 / 5.0_f64 * t5211 * t1289;
    let t12683 = 4.0_f64 / 15.0_f64 * t2076 * t3565;
    let t12684 = t2076 * t3660;
    let t12685 = 8.0_f64 / 45.0_f64 * t12684;
    let t12689 = 16.0_f64 / 15.0_f64 * t1325 * t4829 * t4632 * t940;
    let t12691 = 8.0_f64 / 15.0_f64 * t3745 * t1997;
    let t12693 = t1325 * t3859 * t5413;
    (t12681, t12683, t12685, t12689, t12691, t12693)
}
