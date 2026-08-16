//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1068/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1068(t1730: f64, t2025: f64, t2021: f64, t1179: f64, t4068: f64, t871: f64, t2029: f64, t4119: f64, t224: f64, t4753: f64, t1447: f64, t5176: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11810 = t871 * t1179 * t4068;
    let t11813 = t2029 * t4119;
    let t11821 = t4753 * t224;
    let t11830 = t1447 * t5176;
    (t11796, t11798, t11810, t11813, t11821, t11830)
}
