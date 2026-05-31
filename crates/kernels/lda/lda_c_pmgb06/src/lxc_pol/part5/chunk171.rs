//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 171/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk171<F: Float>(t439: F, t446: F, t109: F, t139: F, t134: F, t138: F, t136: F) -> (F, F, F, F, F) {
    let t448 = t439 * t446 / F::cast_from(45.0_f64);
    let t449 = t109 * t139;
    let t451 = t138 * t449 * t134;
    let t452 = F::cast_from(0.0018891666666666666_f64) * t451;
    let t453 = t139 * t136;
    (t448, t449, t451, t452, t453)
}
