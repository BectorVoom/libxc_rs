//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 806/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk806(t493: f64, t5454: f64, t4161: f64, t4162: f64, t4165: f64, t5393: f64, t5396: f64, t5398: f64, t5419: f64, t5434: f64, t5436: f64, t5438: f64, t5440: f64, t5444: f64, t5446: f64, t5450: f64, t5453: f64) -> (f64, f64) {
    let t5456 = 2.0_f64 / 9.0_f64 * t493 * t5454;
    let t5457 = t5393 - t4161 + 0.06649088888888889_f64 * t4162 + t4165 + t5396 + t5398 + t5419 + t5434 + t5436 + t5438 + t5440 + t5444 - t5446 - t5450 - t5453 - t5456;
    (t5456, t5457)
}
