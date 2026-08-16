//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 899/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk899(t337: f64, t6502: f64, t1919: f64, t493: f64, t2386: f64, t2911: f64, t5470: f64, t1: f64, t1820: f64, t1981: f64, t2599: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6503 = t6502 * t337;
    let t6504 = t1919 * t6503;
    let t6506 = 2.0_f64 / 9.0_f64 * t493 * t6504;
    let t6507 = t2911 * t2386;
    let t6508 = t6507 * t337;
    let t6509 = t5470 * t6508;
    let t6511 = 8.0_f64 / 81.0_f64 * t493 * t6509;
    let t6512 = t1820 * t1;
    let t6513 = t1919 * t6512;
    let t6515 = 4.0_f64 / 27.0_f64 * t1981 * t6513;
    let t6516 = t2599 * t497;
    (t6503, t6504, t6506, t6507, t6508, t6509, t6511, t6512, t6513, t6515, t6516)
}
