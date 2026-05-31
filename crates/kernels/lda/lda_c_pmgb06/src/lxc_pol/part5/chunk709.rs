//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 709/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk709<F: Float>(t337: F, t6502: F, t1919: F, t493: F, t2386: F, t2911: F, t5470: F, t1: F, t1820: F, t1981: F, t2599: F, t497: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6503 = t6502 * t337;
    let t6504 = t1919 * t6503;
    let t6506 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t493 * t6504;
    let t6507 = t2911 * t2386;
    let t6508 = t6507 * t337;
    let t6509 = t5470 * t6508;
    let t6511 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t493 * t6509;
    let t6512 = t1820 * t1;
    let t6513 = t1919 * t6512;
    let t6515 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1981 * t6513;
    let t6516 = t2599 * t497;
    (t6503, t6504, t6506, t6507, t6508, t6509, t6511, t6512, t6513, t6515, t6516)
}
