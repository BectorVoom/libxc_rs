//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1264/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1264<F: Float>(t12868: F, t12870: F, t12878: F, t132: F, t435: F, t6442: F, t137: F, t2604: F, t9610: F, t1512: F, t2606: F, t432: F, t6443: F) -> (F, F, F, F, F, F, F) {
    let t16608 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t12868;
    let t16609 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t12870;
    let t16610 = F::cast_from(8.0_f64) / F::cast_from(243.0_f64) * t12878;
    let t16612 = t132 * t435 * t6442;
    let t16613 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16612;
    let t16617 = t132 * t137 * t9610 * t2604 / F::cast_from(15.0_f64);
    let t16619 = t1512 * t2606 / F::cast_from(15.0_f64);
    let t16621 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t432 * t6443;
    (t16608, t16609, t16610, t16613, t16617, t16619, t16621)
}
