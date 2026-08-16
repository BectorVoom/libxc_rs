//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 457/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk457<F: Float>(t2474: F, t395: F, t382: F, t403: F, t365: F, t305: F) -> (F, F, F, F, F) {
    let t2475 = t395 * t2474;
    let t2478 = t382 * t2474;
    let t2481 = t403 * t2474;
    let t2484 = t365 * t2474;
    let t2487 = t305 * t2474;
    (t2475, t2478, t2481, t2484, t2487)
}
