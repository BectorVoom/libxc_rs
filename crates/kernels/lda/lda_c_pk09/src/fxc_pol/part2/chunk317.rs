//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 317/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk317<F: Float>(t1329: F, t337: F, t281: F, t1214: F, t347: F, t10: F, t1336: F) -> (F, F, F, F) {
    let t1506 = t1329 * t337;
    let t1507 = t1506 * t281;
    let t1510 = t347 * t1214;
    let t1513 = t1336 * t10;
    (t1506, t1507, t1510, t1513)
}
