//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1352/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1352<F: Float>(t188: F, t539: F, t6716: F, t1409: F, t2414: F, t4847: F, t493: F, t6747: F, t4857: F, t6751: F, t1981: F, t4852: F) -> (F, F, F, F, F) {
    let t17787 = t6716 * t539 * t188;
    let t17790 = t2414 * t1409 * t188;
    let t17794 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t493 * t6747 * t4847;
    let t17797 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t493 * t6751 * t4857;
    let t17800 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1981 * t6747 * t4852;
    (t17787, t17790, t17794, t17797, t17800)
}
