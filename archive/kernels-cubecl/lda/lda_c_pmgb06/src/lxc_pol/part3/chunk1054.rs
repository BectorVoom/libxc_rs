//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1054/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1054<F: Float>(t12537: F, t12539: F, t5139: F, t12531: F, t5068: F, t1464: F, t177: F, t1083: F, t1840: F) -> (F, F, F, F) {
    let t12542 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t12537 * t5139 * t12539;
    let t12545 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5068 * t5139 * t12531;
    let t12546 = t177 * t1464;
    let t12547 = t1840 * t1083;
    (t12542, t12545, t12546, t12547)
}
