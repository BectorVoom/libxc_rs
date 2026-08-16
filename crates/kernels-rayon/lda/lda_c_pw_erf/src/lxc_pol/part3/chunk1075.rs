//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1075/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1075(t2178: f64, t3745: f64, t1339: f64, t2176: f64, t348: f64, t519: f64, t1486: f64, t352: f64, t4867: f64, t571: f64, t504: f64, t529: f64) -> (f64, f64, f64, f64) {
    let t12591 = 16.0_f64 / 15.0_f64 * t3745 * t2178;
    let t12595 = 16.0_f64 / 15.0_f64 * t519 * t2176 * t1339 * t348;
    let t12599 = 8.0_f64 / 9.0_f64 * t571 * t4867 * t1486 * t352;
    let t12600 = t529 * t504;
    (t12591, t12595, t12599, t12600)
}
