//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 390/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk390<F: Float>(t1672: F, t472: F, t453: F, t1971: F, t471: F, t1782: F, t1985: F) -> (F, F, F, F) {
    let t2108 = t472 * t1672 / 18.0;
    let t2110 = t453 * t1672 / 18.0;
    let t2111 = t471 * t1971;
    let t2114 = t1985 * t1782;
    (t2108, t2110, t2111, t2114)
}
