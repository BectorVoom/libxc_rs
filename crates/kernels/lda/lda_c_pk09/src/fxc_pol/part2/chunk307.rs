//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 307/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk307<F: Float>(t1362: F, t1363: F, t310: F, t337: F, t381: F, t1284: F) -> (F, F, F, F) {
    let t1364 = t1362 * t1363;
    let t1365 = t310 * t1364;
    let t1368 = t381 * t337;
    let t1369 = t1368 * t1284;
    (t1364, t1365, t1368, t1369)
}
