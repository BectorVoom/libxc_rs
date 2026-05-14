//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 663/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk663<F: Float>(t439: F, t6491: F, t153: F, t1962: F, t1864: F, t4619: F, t1859: F, t2386: F, t2918: F, t337: F, t1919: F, t493: F, t2911: F, t5470: F, t1: F, t1820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6493 = 2.0 / 45.0 * t439 * t6491;
    let t6494 = t1962 * t153;
    let t6495 = t6494 * t1864;
    let t6497 = 4.0 / 45.0 * t439 * t6495;
    let t6498 = t4619 * t153;
    let t6499 = t6498 * t1859;
    let t6501 = 2.0 / 27.0 * t439 * t6499;
    let t6502 = t2918 * t2386;
    let t6503 = t6502 * t337;
    let t6504 = t1919 * t6503;
    let t6506 = 2.0 / 9.0 * t493 * t6504;
    let t6507 = t2911 * t2386;
    let t6508 = t6507 * t337;
    let t6509 = t5470 * t6508;
    let t6511 = 8.0 / 81.0 * t493 * t6509;
    let t6512 = t1820 * t1;
    (t6493, t6494, t6495, t6497, t6498, t6499, t6501, t6502, t6503, t6504, t6506, t6507, t6508, t6509, t6511, t6512)
}
