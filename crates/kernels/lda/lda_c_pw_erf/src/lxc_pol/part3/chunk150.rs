//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 150/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk150<F: Float>(t156: F, t316: F, t337: F, t365: F, t372: F, t380: F, t387: F, t4: F, t71: F, t84: F) -> F {
    let t390 = F::cast_from(0.0005323644333333333_f64) * t4 * t156 * t71 + F::new(1.0) * t365 * t372 - t316 - t337 + F::cast_from(0.0001831155503675316_f64) * t4 * t156 * t84 + F::cast_from(0.5848223397455204_f64) * t380 * t387;
    t390
}
