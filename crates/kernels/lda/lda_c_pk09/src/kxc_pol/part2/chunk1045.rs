//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1045/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1045<F: Float>(t11351: F, t337: F, t430: F, t11059: F, t489: F, t2738: F, t6247: F, t6977: F, t2739: F, t7473: F, t2042: F, t545: F) -> (F, F, F, F, F, F) {
    let t11352 = t11351 * t337;
    let t11353 = t11352 * t430;
    let t11356 = t489 * t11059;
    let t11362 = t6247 * t2738;
    let t11363 = t11362 * t6977;
    let t11366 = t2739 * t7473;
    let t11367 = t11366 * t2042;
    let t11369 = t545 * t11059;
    (t11352, t11353, t11356, t11363, t11367, t11369)
}
