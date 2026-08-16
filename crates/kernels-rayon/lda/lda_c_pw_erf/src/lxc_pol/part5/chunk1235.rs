//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1235/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1235(t12403: f64, t21468: f64, t4488: f64, t12113: f64, t21410: f64, t3974: f64, t4475: f64, t6408: f64, t6413: f64, t6748: f64, t6379: f64, t6752: f64) -> (f64, f64, f64, f64, f64) {
    let t22222 = 16.0_f64 / 15.0_f64 * t4488 * t12403 * t21468;
    let t22225 = 8.0_f64 / 5.0_f64 * t4488 * t12113 * t21410;
    let t22228 = 16.0_f64 / 15.0_f64 * t3974 * t4475 * t6408;
    let t22231 = 16.0_f64 / 5.0_f64 * t3974 * t6748 * t6413;
    let t22234 = 16.0_f64 / 3.0_f64 * t3974 * t6752 * t6379;
    (t22222, t22225, t22228, t22231, t22234)
}
