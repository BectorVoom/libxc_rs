//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 591/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk591<F: Float>(t1238: F, t927: F, t1167: F, t919: F, t921: F, t2381: F, t179: F, t2405: F, t404: F, t326: F, t397: F, t297: F, t401: F, t46: F) -> (F, F, F, F, F, F, F, F) {
    let t3217 = t1238 * t927;
    let t3223 = t1167 * t919;
    let t3224 = t3223 * t921;
    let t3225 = t2381 * t3224;
    let t3229 = t179 * t2405 * t1167;
    let t3230 = t404 * t3229;
    let t3232 = t397 * t326;
    let t3234 = t401 * t297 * t46;
    (t3217, t3223, t3224, t3225, t3229, t3230, t3232, t3234)
}
