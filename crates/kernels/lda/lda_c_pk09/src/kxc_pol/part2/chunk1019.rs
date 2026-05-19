//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1019/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1019<F: Float>(t44: F, t11007: F, t11055: F, t1727: F, t2727: F, t427: F, t11033: F, zeta_threshold: F) -> F {
    let t45 = t44 <= zeta_threshold;
    let t11058 = piecewise3::<F>(t45, t11007, t11055 * t427 + t1727 * t2727);
    let t11059 = t11033 + t11058;
    t11059
}
