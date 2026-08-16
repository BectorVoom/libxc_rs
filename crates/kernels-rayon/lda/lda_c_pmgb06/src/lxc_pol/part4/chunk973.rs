//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 973/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk973(t4317: f64, t707: f64, t1100: f64, t79: f64, t1166: f64, t1183: f64, t297: f64, t301: f64, t4320: f64, t711: f64, t715: f64, t1227: f64, t27: f64, t402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8189 = t707 * t4317;
    let t8193 = t79 * t1100;
    let t8194 = 120.0_f64 * t8193;
    let t8206 = t297 * t1166 * t1183 * t301;
    let t8208 = t4320 * t711;
    let t8211 = 0.7805426614091894_f64 * t4320 * t715;
    let t8228 = t1227 * t27 * t402;
    (t8189, t8193, t8194, t8206, t8208, t8211, t8228)
}
