//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1203/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1203<F: Float>(t130: F, t830: F, t5067: F, t5072: F, t5137: F, t5140: F, t12995: F, t13020: F, t15324: F, t2377: F, t332: F, t477: F) -> (F, F, F, F, F) {
    let t15861 = t830 * t130;
    let t15862 = t15861 * t5067;
    let t15864 = F::new(8.0) / F::new(45.0) * t15862 * t5072;
    let t15865 = t15861 * t5137;
    let t15867 = F::new(4.0) / F::new(27.0) * t15865 * t5140;
    let t15870 = F::new(16.0) / F::new(9.0) * t13020 * t12995 * t15324;
    let t15872 = t2377 * t477 * t332;
    (t15862, t15864, t15867, t15870, t15872)
}
