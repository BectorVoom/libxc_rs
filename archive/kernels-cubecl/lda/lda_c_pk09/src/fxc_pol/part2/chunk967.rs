//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 967/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk967<F: Float>(t1614: F, t9759: F, t327: F, t9819: F, t1625: F, t1610: F, t2474: F, t93: F, t1336: F, t2551: F, t332: F, t9836: F) -> (F, F, F, F, F) {
    let t10262 = t9759 * t1614;
    let t10269 = t327 * t9819;
    let t10270 = t10269 * t1625;
    let t10274 = t1610 * t2474;
    let t10275 = t93 * t10274;
    let t10280 = t2551 * t1336;
    let t10281 = t10280 * t1625;
    let t10287 = t332 * t9836;
    (t10262, t10270, t10275, t10281, t10287)
}
