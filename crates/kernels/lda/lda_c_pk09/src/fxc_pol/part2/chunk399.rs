//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 399/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk399<F: Float>(t2152: F, t95: F, t120: F, t119: F, t2143: F, t63: F, t673: F, t672: F) -> (F, F, F, F, F, F) {
    let t2153 = t95 * t2152;
    let t2154 = t120 * t2153;
    let t2155 = t119 * t2154;
    let t2157 = t63 * t2143;
    let t2158 = t673 * t2157;
    let t2159 = t672 * t2158;
    (t2153, t2154, t2155, t2157, t2158, t2159)
}
