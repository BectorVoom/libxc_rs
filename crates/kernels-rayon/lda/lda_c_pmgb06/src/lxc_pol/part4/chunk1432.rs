//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1432/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1432(t607: f64, t6355: f64, t1710: f64, t2519: f64, t10343: f64, t10346: f64, t10348: f64, t10350: f64, t10353: f64, t10356: f64, t10358: f64, t10362: f64, t17766: f64, t17767: f64, t17768: f64, t17769: f64, t17772: f64) -> f64 {
    let t18329 = t6355 * t607;
    let t18331 = t2519 * t1710;
    let t18333 = -t17766 + t17767 + t17768 + t17769 + t17772 + t10343 / 3.0_f64 + 0.12155555555555556_f64 * t10346 - 2.0_f64 / 27.0_f64 * t10348 - 4.0_f64 / 9.0_f64 * t10350 - 0.027012345679012346_f64 * t10353 - t10356 - t10358 + t10362 - 4.0_f64 / 45.0_f64 * t18329 + 2.0_f64 / 135.0_f64 * t18331;
    t18333
}
