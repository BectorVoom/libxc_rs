//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk803(t1380: f64, t7538: f64, t493: f64, t1897: f64, t7493: f64, t439: f64, t7470: f64, t7472: f64, t7505: f64, t7506: f64, t7507: f64, t7511: f64, t7515: f64, t7519: f64, t7523: f64, t7527: f64, t7529: f64, t7531: f64, t7534: f64, t7537: f64) -> (f64, f64, f64, f64, f64) {
    let t7539 = t1380 * t7538;
    let t7541 = t493 * t7539 / 15.0_f64;
    let t7542 = t1897 * t7493;
    let t7544 = 2.0_f64 / 15.0_f64 * t439 * t7542;
    let t7545 = t7470 + t7472 + t7505 - t7506 - t7507 - t7511 - t7515 + t7519 - t7523 - t7527 - t7529 - t7531 - t7534 - t7537 - t7541 - t7544;
    (t7539, t7541, t7542, t7544, t7545)
}
