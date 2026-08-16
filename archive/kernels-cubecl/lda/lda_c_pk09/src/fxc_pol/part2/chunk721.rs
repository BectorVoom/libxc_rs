//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 721/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk721<F: Float>(t2035: F, t7292: F, t2000: F, t462: F, t6196: F, t337: F, t461: F, t429: F, t2042: F, t1819: F, t7286: F, t450: F) -> (F, F, F, F, F, F, F) {
    let t7293 = t2035 * t7292;
    let t7296 = t462 * t2000;
    let t7297 = t7296 * t6196;
    let t7299 = t461 * t337;
    let t7300 = t7299 * t429;
    let t7301 = t2035 * t7300;
    let t7302 = t7301 * t2042;
    let t7304 = t1819 * t7286;
    let t7307 = t450 * t337;
    (t7293, t7296, t7297, t7300, t7302, t7304, t7307)
}
