//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 706/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk706<F: Float>(t5: F, t2249: F, t4359: F, t3537: F, t760: F, t1: F, t1212: F, t332: F, t395: F, t1069: F, t1074: F, t2192: F, t2195: F, t247: F, t330: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t4360 = t4359 * t2249;
    let t4363 = t3537 * t760;
    let t4366 = t1212 * t1;
    let t4367 = t332 * t395;
    let t4377 = piecewise3::<F>(t6, F::new(0.0), F::new(8.0) / F::new(27.0) * t4363 * t1069 - F::new(8.0) / F::new(9.0) * t4366 * t4367 - F::new(2.0) / F::new(9.0) * t2192 * t1074 + F::new(4.0) / F::new(3.0) * t330 * t395 - F::new(4.0) * t2195 * t247);
    (t4360, t4363, t4367, t4377)
}
