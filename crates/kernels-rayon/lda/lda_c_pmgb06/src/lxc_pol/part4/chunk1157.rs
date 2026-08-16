//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1157/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1157(t1080: f64, t6502: f64, t1915: f64, t493: f64, t1602: f64, t2545: f64, t2871: f64, t12633: f64, t439: f64, t5364: f64, t5344: f64, t5482: f64) -> (f64, f64, f64, f64, f64) {
    let t15223 = t6502 * t1080;
    let t15226 = 8.0_f64 / 15.0_f64 * t493 * t1915 * t15223;
    let t15230 = 4.0_f64 / 45.0_f64 * t493 * t2871 * t2545 * t1602;
    let t15233 = 4.0_f64 / 45.0_f64 * t439 * t12633 * t5364;
    let t15236 = 4.0_f64 / 45.0_f64 * t439 * t5482 * t5344;
    (t15223, t15226, t15230, t15233, t15236)
}
