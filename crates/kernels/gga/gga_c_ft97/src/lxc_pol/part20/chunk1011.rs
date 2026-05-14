//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1011/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1011<F: Float>(t1434: F, t24508: F, t681: F, t2399: F, t6109: F, t6111: F, t24473: F, t24503: F, t1636: F, t6144: F, t89: F, t1439: F, t2999: F, t24223: F, t5996: F, t24744: F, t8392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97220 = t1434 * t681 * t24508;
    let t97232 = t6109 * t2399 * t6111;
    let t97235 = t6109 * t681 * t24473;
    let t97238 = t6109 * t681 * t24503;
    let t97244 = t89 * t1636 * t6144;
    let t97247 = t89 * t2999 * t1439;
    let t97248 = 28.0 / 27.0 * t97247;
    let t97255 = t5996 * t24223;
    let t97259 = t8392 * t24744;
    (t97220, t97232, t97235, t97238, t97244, t97247, t97248, t97255, t97259)
}
