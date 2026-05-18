//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 956/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk956<F: Float>(t6486: F, t6488: F, t6490: F, t6493: F, t6497: F, t6501: F, t6506: F, t6511: F, t6515: F, t6520: F, t6526: F, t6530: F, t6532: F, t6535: F, t6538: F) -> F {
    let t7204 = -t6486 - t6488 + t6490 - t6493 - t6497 + t6501 - t6506 + t6511 - t6515 + t6520 + t6526 + t6530 + t6532 + t6535 - t6538;
    t7204
}
