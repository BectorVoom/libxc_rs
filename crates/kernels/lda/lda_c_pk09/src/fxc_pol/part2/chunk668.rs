//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 668/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk668<F: Float>(t1853: F, t6196: F, t1777: F, t1947: F, t2042: F, t1931: F, t1943: F, t1240: F, t1906: F, t1905: F, t1948: F, t1920: F) -> (F, F, F, F, F, F) {
    let t6197 = t1853 * t6196;
    let t6199 = t1777 * t1947;
    let t6200 = t6199 * t2042;
    let t6210 = t1931 * t6196;
    let t6212 = t1943 * t1947;
    let t6213 = t6212 * t2042;
    let t6215 = t1906 * t1240;
    let t6216 = t1905 * t6215;
    let t6217 = t1948 * t6216;
    let t6223 = t1920 * t1240;
    (t6197, t6200, t6210, t6213, t6217, t6223)
}
