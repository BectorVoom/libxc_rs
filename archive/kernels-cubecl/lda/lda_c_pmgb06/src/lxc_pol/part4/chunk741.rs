//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 741/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk741<F: Float>(t4801: F, t529: F, t166: F, t161: F, t1887: F, t436: F, t1928: F, t432: F, t1873: F, t435: F, t132: F, t1517: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t4802 = t4801 * t529;
    let t4803 = t166 * t4802;
    let t4805 = t161 * t4803 / F::cast_from(15.0_f64);
    let t4807 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1887 * t436;
    let t4809 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t432 * t1928;
    let t4810 = t435 * t1873;
    let t4812 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t132 * t4810;
    let t4814 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t802 * t1517;
    (t4802, t4803, t4805, t4807, t4809, t4810, t4812, t4814)
}
