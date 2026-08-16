//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 700/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk700(t1915: f64, t6402: f64, t493: f64, t1: f64, t1825: f64, t1981: f64, t1420: f64, t2501: f64, t2578: f64, t477: f64, t1385: f64, t439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6403 = t1915 * t6402;
    let t6405 = 2.0_f64 / 15.0_f64 * t493 * t6403;
    let t6406 = t1825 * t1;
    let t6407 = t1915 * t6406;
    let t6409 = 8.0_f64 / 45.0_f64 * t1981 * t6407;
    let t6411 = 2.0_f64 / 45.0_f64 * t1420 * t2501;
    let t6412 = t2578 * t477;
    let t6413 = t1385 * t6412;
    let t6415 = t439 * t6413 / 45.0_f64;
    (t6403, t6405, t6406, t6407, t6409, t6411, t6412, t6413, t6415)
}
