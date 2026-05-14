//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1061/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1061<F: Float>(t23052: F, t641: F, t746: F, t741: F, t24001: F, t5322: F, t5321: F, t24189: F, t7311: F, t24246: F, t24249: F, t24252: F, t24255: F, t24258: F, t24261: F, t24264: F, t24266: F) -> (F, F, F, F, F, F, F) {
    let t24268 = t641 * t23052;
    let t24269 = t746 * t24268;
    let t24270 = t741 * t24269;
    let t24272 = t5322 * t24001;
    let t24273 = t5321 * t24272;
    let t24275 = t7311 * t24189;
    let t24276 = t5321 * t24275;
    let t24278 = t24246 / 6.0 + 19.0 / 144.0 * t24249 - t24252 / 24.0 - t24255 / 128.0 + t24258 / 8.0 - 2.0 / 3.0 * t24261 - t24264 / 288.0 - t24266 / 16.0 - t24270 / 192.0 + t24273 / 96.0 + t24276 / 288.0;
    (t24269, t24270, t24272, t24273, t24275, t24276, t24278)
}
