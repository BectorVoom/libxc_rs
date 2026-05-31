//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 798/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk798<F: Float>(t1423: F, t1908: F, t1069: F, t1531: F, t822: F, t1385: F, t439: F, t1898: F, t1897: F, t4663: F, t1902: F, t1447: F, t1925: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5342 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t1423 * t1908;
    let t5344 = t822 * t1531 * t1069;
    let t5345 = t1385 * t5344;
    let t5347 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t439 * t5345;
    let t5349 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t1423 * t1898;
    let t5350 = t1897 * t4663;
    let t5352 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t5350;
    let t5354 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t1423 * t1902;
    let t5356 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t1447 * t1925;
    (t5342, t5344, t5345, t5347, t5349, t5350, t5352, t5354, t5356)
}
