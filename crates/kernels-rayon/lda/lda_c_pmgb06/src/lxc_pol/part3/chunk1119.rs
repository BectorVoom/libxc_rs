//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1119/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1119(t10109: f64, t10111: f64, t10113: f64, t1555: f64, t1848: f64, t13277: f64, t13279: f64, t13283: f64, t13284: f64, t13285: f64, t13286: f64, t13287: f64) -> (f64, f64, f64, f64, f64) {
    let t13288 = 2.0_f64 / 135.0_f64 * t10109;
    let t13289 = 2.0_f64 / 135.0_f64 * t10111;
    let t13290 = 16.0_f64 / 243.0_f64 * t10113;
    let t13291 = t1848 * t1555;
    let t13292 = t13291 / 45.0_f64;
    let t13293 = -t13277 - t13279 - t13283 - t13284 - t13285 + t13286 + t13287 - t13288 + t13289 + t13290 - t13292;
    (t13288, t13289, t13290, t13292, t13293)
}
