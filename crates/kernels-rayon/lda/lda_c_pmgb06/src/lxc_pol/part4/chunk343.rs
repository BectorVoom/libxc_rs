//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 343/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk343(t1193: f64, t98: f64, t115: f64, t569: f64, t1072: f64, t1105: f64, t1185: f64, t1189: f64, t1192: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1194 = t1193 * t98;
    let t1195 = t569 * t115;
    let t1197 = 0.00786258_f64 * t1194 * t1195;
    let t1198 = 4.0_f64 * t1072;
    let t1199 = 3.0_f64 * t1105;
    let t1200 = t1185 + t1189 - t1192 + t1197 + t81 - t1198 + t1199;
    (t1194, t1195, t1197, t1198, t1199, t1200)
}
