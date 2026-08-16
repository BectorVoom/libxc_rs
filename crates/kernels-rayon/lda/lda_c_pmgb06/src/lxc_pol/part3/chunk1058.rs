//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1058/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1058(t1444: f64, t5467: f64, t5471: f64, t4880: f64, t493: f64, t5463: f64, t10220: f64, t176: f64, t4885: f64, t1820: f64, t2938: f64, t1919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12574 = t1444 * t5467 / 9.0_f64;
    let t12576 = 8.0_f64 / 27.0_f64 * t1444 * t5471;
    let t12579 = t493 * t5463 * t4880 / 9.0_f64;
    let t12580 = t10220 * t176;
    let t12583 = 8.0_f64 / 27.0_f64 * t493 * t12580 * t4885;
    let t12584 = t1820 * t2938;
    let t12587 = t493 * t1919 * t12584 / 27.0_f64;
    (t12574, t12576, t12579, t12583, t12584, t12587)
}
