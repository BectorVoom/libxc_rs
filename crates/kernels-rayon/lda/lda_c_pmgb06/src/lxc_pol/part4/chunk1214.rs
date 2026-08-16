//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1214/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1214(t1420: f64, t6361: f64, t1972: f64, t5337: f64, t1080: f64, t6507: f64, t1919: f64, t493: f64, t4602: f64, t6407: f64, t1981: f64, t5447: f64, t6406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16000 = 4.0_f64 / 45.0_f64 * t1420 * t6361;
    let t16002 = 4.0_f64 / 45.0_f64 * t1972 * t5337;
    let t16003 = t6507 * t1080;
    let t16006 = 4.0_f64 / 3.0_f64 * t493 * t1919 * t16003;
    let t16008 = 16.0_f64 / 45.0_f64 * t4602 * t6407;
    let t16011 = 16.0_f64 / 45.0_f64 * t1981 * t5447 * t6406;
    (t16000, t16002, t16003, t16006, t16008, t16011)
}
