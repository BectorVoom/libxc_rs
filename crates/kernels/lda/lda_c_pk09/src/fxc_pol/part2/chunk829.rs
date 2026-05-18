//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 829/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk829<F: Float>(t2269: F, t733: F, t204: F, t737: F, t2152: F, t823: F, t825: F, t96: F, t8092: F, t831: F, t957: F, t2318: F, t623: F, t844: F) -> (F, F, F, F, F, F) {
    let t8392 = t2269 * t733;
    let t8393 = t8392 * t204;
    let t8394 = t8393 * t737;
    let t8404 = t96 * t2152 * t823 * t825;
    let t8407 = t831 * t8092;
    let t8413 = t957 * t8092;
    let t8416 = t844 * t2318 * t623;
    (t8392, t8394, t8404, t8407, t8413, t8416)
}
