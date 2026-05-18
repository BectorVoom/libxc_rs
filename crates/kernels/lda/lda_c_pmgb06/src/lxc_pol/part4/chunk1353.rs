//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1353/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1353<F: Float>(t1912: F, t5194: F, t5305: F, t1972: F, t4728: F, t4732: F, t1447: F, t6752: F, t1444: F, t6744: F, t12012: F, t1911: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t17801 = t5194 * t1912;
    let t17802 = F::new(8.0) / F::new(135.0) * t17801;
    let t17804 = F::new(4.0) / F::new(45.0) * t5305 * t1912;
    let t17806 = F::new(4.0) / F::new(45.0) * t1972 * t4728;
    let t17808 = F::new(2.0) / F::new(45.0) * t1972 * t4732;
    let t17809 = t1447 * t6752;
    let t17810 = F::new(8.0) / F::new(81.0) * t17809;
    let t17812 = F::new(4.0) / F::new(45.0) * t1444 * t6744;
    let t17815 = F::new(4.0) / F::new(45.0) * t493 * t12012 * t1911;
    (t17802, t17804, t17806, t17808, t17810, t17812, t17815)
}
