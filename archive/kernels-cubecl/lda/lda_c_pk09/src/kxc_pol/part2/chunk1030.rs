//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1030/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1030<F: Float>(t11059: F, t501: F, t524: F, t305: F, t68: F, t11092: F, t1798: F, t1240: F, t2889: F, t6267: F, t93: F, t1729: F) -> (F, F, F, F, F, F, F) {
    let t11122 = t501 * t11059;
    let t11125 = t524 * t11059;
    let t11128 = t305 * t11059;
    let t11129 = t11128 * t68;
    let t11134 = t1798 * t11092;
    let t11140 = t2889 * t1240;
    let t11142 = t6267 * t93 * t11140;
    let t11144 = t2889 * t1729;
    (t11122, t11125, t11128, t11129, t11134, t11142, t11144)
}
