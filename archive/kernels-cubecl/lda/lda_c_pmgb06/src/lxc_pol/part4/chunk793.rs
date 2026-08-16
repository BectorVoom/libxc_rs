//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 793/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk793<F: Float>(t3296: F, t2042: F, t435: F, t132: F, t1847: F, t224: F) -> (F, F, F, F) {
    let t5301 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t3296;
    let t5302 = t435 * t2042;
    let t5304 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t132 * t5302;
    let t5305 = t1847 * t224;
    (t5301, t5302, t5304, t5305)
}
