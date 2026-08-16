//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 811/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk811<F: Float>(t1436: F, t7645: F, t439: F, t2002: F, t2493: F, t1962: F, t2492: F, t1972: F, t2489: F, t1988: F, t2488: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t7646 = t1436 * t7645;
    let t7648 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t439 * t7646;
    let t7650 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t2493;
    let t7651 = t1962 * t2492;
    let t7653 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t7651;
    let t7655 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t2489;
    let t7656 = t1988 * t2488;
    let t7658 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t7656;
    (t7646, t7648, t7650, t7651, t7653, t7655, t7656, t7658)
}
