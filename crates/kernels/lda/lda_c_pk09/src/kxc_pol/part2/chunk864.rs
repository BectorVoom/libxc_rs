//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 864/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk864<F: Float>(t8911: F, t8918: F, t8927: F, t8942: F, t61: F, t825: F, t96: F, t2143: F, t844: F, t873: F, t2251: F, t748: F) -> (F, F, F) {
    let t8944 = t8911 + t8918 + t8927 + t8942;
    let t8947 = t96 * t61 * t8944 * t825;
    let t8953 = t844 * t873 * t2143;
    let t8964 = t748 * t2251;
    (t8947, t8953, t8964)
}
