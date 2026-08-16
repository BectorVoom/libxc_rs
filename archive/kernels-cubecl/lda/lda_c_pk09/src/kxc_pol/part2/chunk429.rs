//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 429/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk429<F: Float>(t133: F, t2192: F, t2143: F, t742: F, t2149: F, t947: F, t131: F, t2152: F, t707: F, t121: F, t409: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2193 = t133 * t2192;
    let t2197 = t133 * t2143;
    let t2198 = t742 * t2197;
    let t2201 = t947 * t2149;
    let t2202 = t131 * t2201;
    let t2205 = t707 * t2152;
    let t2206 = t131 * t2205;
    let t2209 = t121 * t2143;
    let t2210 = t409 * t2209;
    (t2193, t2197, t2198, t2201, t2202, t2205, t2206, t2209, t2210)
}
