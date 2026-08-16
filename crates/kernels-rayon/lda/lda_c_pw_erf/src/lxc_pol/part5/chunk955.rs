//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 955/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk955(t108: f64, t2103: f64, t267: f64, t219: f64, t4048: f64, t473: f64, t10467: f64, t2030: f64, t519: f64, t518: f64, t5214: f64, t3663: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t12143 = t2103 * t108 * t267;
    let t12158 = t473 * t4048 * t219;
    let t12196 = t519 * t10467 * t2030;
    let t12197 = 8.0_f64 / 135.0_f64 * t12196;
    let t12299 = t5214 * t518;
    let t12309 = t822 * t3663;
    (t12143, t12158, t12197, t12299, t12309)
}
