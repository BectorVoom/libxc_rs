//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 54/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk54<F: Float>(t103: F, t107: F, t110: F, t3: F, t34: F, t55: F, t93: F) -> F {
    let t113 = F::cast_from(1.0_f64) - t93 * t55 * t34 / F::cast_from(4.0_f64) + F::cast_from(0.0204825_f64) * t103 - F::cast_from(0.0030486129349252553_f64) * t3 + F::cast_from(0.0003485625_f64) * t107 * t110;
    t113
}
