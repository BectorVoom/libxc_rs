//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1266/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1266<F: Float>(t1420: F, t6475: F, t12139: F, t439: F, t6151: F, t16608: F, t16609: F, t16610: F, t16613: F, t16617: F, t16619: F, t16621: F, t16623: F, t16625: F, t16629: F, t16631: F, t16633: F, t16635: F) -> (F, F, F) {
    let t16637 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t1420 * t6475;
    let t16640 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t439 * t12139 * t6151;
    let t16641 = t16608 + t16609 - t16610 + t16613 + t16617 + t16619 + t16621 + t16623 + t16625 + t16629 + t16631 + t16633 + t16635 + t16637 + t16640;
    (t16637, t16640, t16641)
}
