//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 630/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk630<F: Float>(t1291: F, t1296: F, t1297: F, t1309: F, t3620: F, t3622: F, t3625: F, t3632: F, t3633: F, t3636: F, t3656: F, t378: F, t384: F, t74: F) -> F {
    let t3658 = -F::cast_from(3.0_f64) * t1291 * t1309 + F::cast_from(6.0_f64) * t1296 * t3636 + F::cast_from(6.0_f64) * t3625 * t1297 + t3620 * t74 - F::cast_from(3.0_f64) * t3622 * t384 - F::cast_from(6.0_f64) * t3632 * t3633 - t378 * t3656;
    t3658
}
