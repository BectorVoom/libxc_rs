//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 883/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk883<F: Float>(t2277: F, t861: F, t356: F, t2280: F, t364: F, t2288: F, t881: F, t2317: F, t877: F, t2249: F, t862: F, t2278: F, t858: F, t361: F, t2196: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6287 = 1.0 / t2277 / t861;
    let t6288 = t356 * t6287;
    let t6290 = 1.0 / t2280 / t364;
    let t6294 = t2288 * t881;
    let t6300 = t877 * t2317;
    let t6303 = t2249 * t862;
    let t6308 = t858 * t2278;
    let t6312 = 1.0 / t2277 / t361;
    let t6313 = t356 * t6312;
    let t6317 = t828 * t2196;
    (t6287, t6288, t6290, t6294, t6300, t6303, t6308, t6312, t6313, t6317)
}
