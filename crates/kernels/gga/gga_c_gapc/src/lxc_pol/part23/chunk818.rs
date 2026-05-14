//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 818/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk818<F: Float>(t11233: F, t11278: F, t209: F, t3655: F, t575: F, t687: F, t1049: F, t8598: F, t2967: F, t8601: F, t2964: F, t3179: F, t3659: F, t4908: F, t4915: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11279 = t11233 + t11278;
    let t11280 = t11279 * t209;
    let t11281 = t3655 * t575;
    let t11282 = t11281 * t687;
    let t11283 = t8598 * t1049;
    let t11284 = 2.0 * t11283;
    let t11285 = t8601 * t2967;
    let t11286 = 4.0 * t11285;
    let t11287 = t2964 * t3179;
    let t11288 = 2.0 * t11287;
    let t11289 = t4908 * t3659;
    let t11290 = 2.0 * t11289;
    let t11291 = t3659 * t687;
    let t11292 = t4915 * t11291;
    let t11293 = 6.0 * t11292;
    let t11294 = t1049 * t3179;
    (t11279, t11280, t11281, t11282, t11283, t11284, t11285, t11286, t11287, t11288, t11289, t11290, t11291, t11292, t11293, t11294)
}
