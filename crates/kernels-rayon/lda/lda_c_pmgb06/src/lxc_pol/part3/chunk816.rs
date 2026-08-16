//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk816(t3391: f64, t3392: f64, t3395: f64, t5396: f64, t5398: f64, t5419: f64, t5434: f64, t5436: f64, t5438: f64, t5440: f64, t5444: f64, t5446: f64, t5450: f64, t5453: f64, t5456: f64) -> f64 {
    let t5685 = t3391 + 16.0_f64 / 3.0_f64 * t3392 + t3395 + t5396 + t5398 + t5419 + t5434 + t5436 + t5438 + t5440 + t5444 - t5446 - t5450 - t5453 - t5456;
    t5685
}
