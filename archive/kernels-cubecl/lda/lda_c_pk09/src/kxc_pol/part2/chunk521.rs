//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 521/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk521<F: Float>(t3141: F, t708: F, t10: F, t125: F, t770: F, t767: F, t720: F) -> (F, F, F, F) {
    let t3142 = t3141 * t708;
    let t3146 = t770 * t125 * t10;
    let t3147 = t767 * t3146;
    let t3148 = t3141 * t720;
    (t3142, t3146, t3147, t3148)
}
