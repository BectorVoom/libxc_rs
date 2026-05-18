//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 719/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk719<F: Float>(t2042: F, t7255: F, t470: F, t902: F, t633: F, t93: F, t1841: F, t1985: F, t1729: F, t2115: F, t1468: F, t1941: F) -> (F, F, F, F, F, F) {
    let t7256 = t7255 * t2042;
    let t7260 = t902 * t470;
    let t7261 = t7260 * t633;
    let t7262 = t93 * t7261;
    let t7267 = t1985 * t1841;
    let t7268 = t2115 * t1729;
    let t7269 = t93 * t7268;
    let t7272 = t1941 * t1468;
    (t7256, t7260, t7262, t7267, t7269, t7272)
}
