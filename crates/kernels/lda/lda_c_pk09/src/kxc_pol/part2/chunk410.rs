//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 410/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk410<F: Float>(t2171: F, t2175: F, t2179: F, t613: F, t617: F, t188: F, t2192: F, t659: F, t702: F, t89: F, t2143: F, t61: F) -> (F, F, F, F, F) {
    let t2233 = t613 + t617 + 0.9421211958699838 * t2171 + 0.9421211958699838 * t2175 - 0.9421211958699838 * t2179;
    let t2237 = t2233 * t188 - t659 * t2192 / 2.0;
    let t2238 = t2237 * t702;
    let t2239 = t2238 * t89;
    let t2246 = t61 * t2143;
    (t2233, t2237, t2238, t2239, t2246)
}
