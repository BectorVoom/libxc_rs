//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1035/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1035<F: Float>(t2149: F, t633: F, t6938: F, t1904: F, t11068: F, t6814: F, t6818: F, t142: F, t480: F, t4007: F, t92: F, t1240: F) -> (F, F, F, F, F) {
    let t11206 = t2149 * t633;
    let t11207 = t6938 * t11206;
    let t11208 = t1904 * t11207;
    let t11211 = t6818 * t6814 * t11068;
    let t11213 = t480 * t142;
    let t11214 = t92 * t4007;
    let t11216 = t11213 * t11214 * t11206;
    let t11218 = t2149 * t1240;
    (t11206, t11208, t11211, t11216, t11218)
}
