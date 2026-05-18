//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1025/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1025<F: Float>(t11327: F, t898: F, t398: F, t19: F, t297: F, t326: F, t397: F, t10115: F, t1167: F, t2888: F, t1227: F, t3874: F) -> (F, F, F, F, F, F) {
    let t11329 = F::new(0.10389515463408878255e3) * t898 * t11327;
    let t11333 = t398 * t398;
    let t11335 = F::new(1.0) / t19 / t11333;
    let t11338 = t397 * t326 * t11335 * t297;
    let t11341 = t10115 * t1167;
    let t11342 = t2888 * t11341;
    let t11345 = t3874 * t1227;
    (t11329, t11335, t11338, t11341, t11342, t11345)
}
