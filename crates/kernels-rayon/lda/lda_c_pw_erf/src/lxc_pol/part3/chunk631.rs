//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 631/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk631(t197: f64, t3476: f64, t2954: f64, t1459: f64, t519: f64, t1497: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t3722 = t197 * t3476;
    let t3723 = t3722 * t2954;
    let t3724 = t1459 * t3723;
    let t3726 = 8.0_f64 / 9.0_f64 * t519 * t3724;
    let t3727 = t1497 * t518;
    (t3723, t3724, t3726, t3727)
}
