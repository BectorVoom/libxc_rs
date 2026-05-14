//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 649/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk649<F: Float>(t7274: F, t93: F, t7273: F, t471: F, t6700: F, t2042: F, t1817: F, t2052: F, t429: F, t2036: F, t2035: F, t2000: F, t462: F, t6196: F, t337: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7275 = t93 * t7274;
    let t7276 = t7273 * t7275;
    let t7278 = t471 * t6700;
    let t7279 = t7278 * t2042;
    let t7283 = t1817 * t1817;
    let t7284 = 1.0 / t7283;
    let t7286 = t2052 * t429;
    let t7292 = t2036 * t429;
    let t7293 = t2035 * t7292;
    let t7296 = t462 * t2000;
    let t7297 = t7296 * t6196;
    let t7299 = t461 * t337;
    (t7275, t7276, t7279, t7284, t7286, t7292, t7293, t7296, t7297, t7299)
}
