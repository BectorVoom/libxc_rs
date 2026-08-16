//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 955/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk955(t5117: f64, t5126: f64, t5650: f64, t5654: f64, t5656: f64, t5658: f64, t6455: f64, t6457: f64, t6459: f64, t6463: f64, t6467: f64, t6474: f64, t6477: f64, t6480: f64, t6482: f64, t6484: f64) -> f64 {
    let t7200 = 0.022363485482220676_f64 * t5650 + t5654 + 0.4328416544945937_f64 * t5656 + 0.1442805514981979_f64 * t5658 - t5117 - t5126 + t6455 - t6457 - t6459 - t6463 - t6467 - t6474 + t6477 + t6480 - t6482 + t6484;
    t7200
}
