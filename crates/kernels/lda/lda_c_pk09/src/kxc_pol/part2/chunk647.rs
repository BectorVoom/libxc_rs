//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 647/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk647<F: Float>(t2115: F, t747: F, t2114: F, t1672: F, t2111: F, t2085: F, t2091: F, t2088: F, t451: F, t6700: F, t2042: F, t1947: F, t2084: F, t2083: F, t305: F, t462: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7223 = t747 * t2115;
    let t7224 = t2114 * t7223;
    let t7226 = t2111 * t1672;
    let t7228 = t2085 * t1672;
    let t7230 = t2091 * t1672;
    let t7232 = t2088 * t1672;
    let t7240 = t451 * t6700;
    let t7241 = t7240 * t2042;
    let t7243 = t2084 * t1947;
    let t7244 = t7243 * t2042;
    let t7248 = t2083 * t305;
    let t7252 = t462 * t6700;
    (t7223, t7224, t7226, t7228, t7230, t7232, t7241, t7244, t7248, t7252)
}
