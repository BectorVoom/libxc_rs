//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1400/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1400<F: Float>(t12224: F, t12227: F, t16237: F, t16239: F, t16242: F, t16243: F, t16244: F, t16248: F, t16250: F, t16252: F, t16255: F, t16259: F, t9478: F, t9481: F, t9483: F) -> F {
    let t18218 = t9478 + t9481 + F::new(0.36466666666666664) * t9483 - F::new(8.0) / F::new(27.0) * t12224 - F::new(4.0) / F::new(9.0) * t12227 + t16237 + t16239 + t16242 + t16243 + t16244 - t16248 + t16250 + t16252 + t16255 + t16259;
    t18218
}
