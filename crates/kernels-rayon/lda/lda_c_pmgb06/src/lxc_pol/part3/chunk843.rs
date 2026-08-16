//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 843/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk843(t1166: f64, t1183: f64, t297: f64, t301: f64, t4320: f64, t711: f64, t715: f64, t110: f64, t3526: f64, t360: f64, t1227: f64, t27: f64, t402: f64) -> (f64, f64, f64, f64, f64) {
    let t8206 = t297 * t1166 * t1183 * t301;
    let t8208 = t4320 * t711;
    let t8211 = 0.7805426614091894_f64 * t4320 * t715;
    let t8220 = t360 * t110 * t3526;
    let t8228 = t1227 * t27 * t402;
    (t8206, t8208, t8211, t8220, t8228)
}
