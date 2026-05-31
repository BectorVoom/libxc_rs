//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1285/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1285<F: Float>(t16884: F, t13139: F, t337: F, t529: F, t6560: F, t12691: F, t5068: F, t13064: F, t5138: F, t13177: F, t1083: F, t2871: F, t493: F, t6516: F) -> (F, F, F, F, F, F) {
    let t16885 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16884;
    let t16886 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13139;
    let t16888 = t6560 * t529 * t337;
    let t16891 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5068 * t12691 * t16888;
    let t16894 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5138 * t13064 * t16888;
    let t16895 = F::cast_from(16.0_f64) / F::cast_from(1215.0_f64) * t13177;
    let t16899 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t493 * t2871 * t6516 * t1083;
    (t16885, t16886, t16891, t16894, t16895, t16899)
}
