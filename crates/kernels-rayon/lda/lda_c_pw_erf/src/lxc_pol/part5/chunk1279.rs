//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1279/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1279(t1308: f64, t571: f64, t593: f64, t7404: f64, t1319: f64, t21820: f64, t2017: f64, t21825: f64, t2334: f64, t811: f64, t11914: f64, t352: f64, t3974: f64) -> (f64, f64, f64, f64, f64) {
    let t22915 = 4.0_f64 / 45.0_f64 * t571 * t1308 * t7404 * t593;
    let t22918 = 8.0_f64 / 45.0_f64 * t571 * t1319 * t21820;
    let t22921 = 4.0_f64 / 27.0_f64 * t571 * t2017 * t21825;
    let t22922 = t2334 * t811;
    let t22926 = 64.0_f64 / 27.0_f64 * t3974 * t11914 * t22922 * t352;
    (t22915, t22918, t22921, t22922, t22926)
}
