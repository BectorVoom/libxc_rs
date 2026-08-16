//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 699/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk699(t2496: f64, t2979: f64, t493: f64, t2088: f64, t838: f64, t1380: f64, t1831: f64, t851: f64, t1981: f64, t2545: f64, t529: f64, t2541: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6387 = t2979 * t2496;
    let t6389 = 2.0_f64 / 45.0_f64 * t493 * t6387;
    let t6390 = t838 * t2088;
    let t6391 = t1380 * t6390;
    let t6393 = 2.0_f64 / 45.0_f64 * t493 * t6391;
    let t6394 = t1831 * t851;
    let t6395 = t1380 * t6394;
    let t6397 = 4.0_f64 / 45.0_f64 * t1981 * t6395;
    let t6398 = t2545 * t529;
    let t6399 = t1380 * t6398;
    let t6401 = 2.0_f64 / 45.0_f64 * t493 * t6399;
    let t6402 = t2541 * t337;
    (t6387, t6389, t6390, t6391, t6393, t6394, t6395, t6397, t6398, t6399, t6401, t6402)
}
