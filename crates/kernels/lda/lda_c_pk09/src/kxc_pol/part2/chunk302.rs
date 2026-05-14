//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 302/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk302<F: Float>(t130: F, t1397: F, t93: F, t337: F, t394: F, t1284: F) -> (F, F, F, F) {
    let t1398 = t130 * t1397;
    let t1399 = t93 * t1398;
    let t1402 = t394 * t337;
    let t1403 = t1402 * t1284;
    (t1398, t1399, t1402, t1403)
}
