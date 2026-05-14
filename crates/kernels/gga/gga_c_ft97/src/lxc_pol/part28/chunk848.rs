//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 848/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk848<F: Float>(t1286: F, t23047: F, t32026: F, t32029: F, t1774: F, t5700: F, t7151: F, t1526: F, t5692: F, t7705: F, t1322: F, t8281: F, t32399: F, t5495: F, t31998: F, t32391: F, t376: F) -> (F, F, F, F, F, F, F, F) {
    let t137404 = t1286 * t23047;
    let t137412 = t32026 * t32029;
    let t137415 = t7151 * t1774 * t5700;
    let t137418 = t1526 * t7705 * t5692;
    let t137442 = 2.0 / 27.0 * t7151 * t8281 * t1322;
    let t137463 = t5495 * t32399;
    let t137471 = t5495 * t31998;
    let t137476 = t1286 * t376 * t32391;
    (t137404, t137412, t137415, t137418, t137442, t137463, t137471, t137476)
}
