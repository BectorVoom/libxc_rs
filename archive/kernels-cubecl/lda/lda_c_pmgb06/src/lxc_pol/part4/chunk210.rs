//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 210/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk210<F: Float>(t208: F, t580: F, t213: F, t206: F, t97: F, t29: F, t31: F, t101: F) -> (F, F, F, F, F) {
    let t581 = t580 * t208;
    let t583 = t581 * t213 / F::cast_from(3.0_f64);
    let t584 = t206 * t97;
    let t586 = F::cast_from(1.0_f64) / t29 * t31;
    let t588 = t586 * t101 * t208;
    (t581, t583, t584, t586, t588)
}
