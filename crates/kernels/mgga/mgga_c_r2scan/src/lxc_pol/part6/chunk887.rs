//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 887/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk887<F: Float>(t2124: F, t495: F, t5167: F, t1234: F, t537: F, t2252: F, t277: F, t360: F, t20: F, t489: F, t524: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t6278 = t2124 * t5167 * t495;
    let t6281 = t537 * t1234;
    let t6283 = t2124 * t6281 * t495;
    let t6286 = t277 * t2252;
    let t6287 = t6286 * t495;
    let t6288 = t360 * t6287;
    let t6291 = t489 * t20;
    let t6293 = t524 * t525 * t6291;
    (t6278, t6283, t6286, t6287, t6288, t6291, t6293)
}
