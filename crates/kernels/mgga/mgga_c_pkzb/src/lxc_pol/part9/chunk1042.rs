//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1042/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1042<F: Float>(t178: F, t8358: F, t2364: F, t2394: F, t2886: F, t980: F, t6517: F, t919: F, t1227: F, t2411: F, t300: F, t1235: F, t297: F, t46: F) -> (F, F, F, F, F, F) {
    let t10043 = t8358 * t178;
    let t10044 = t2364 * t10043;
    let t10047 = t2394 * t10043;
    let t10063 = t980 * t2886;
    let t10121 = t6517 * t919;
    let t10212 = t2411 * t1227;
    let t10213 = t300 * t10212;
    let t10257 = t1235 * t297 * t46;
    (t10044, t10047, t10063, t10121, t10213, t10257)
}
