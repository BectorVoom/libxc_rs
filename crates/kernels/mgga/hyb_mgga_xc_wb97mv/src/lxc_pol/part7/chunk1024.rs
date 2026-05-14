//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1024/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1024<F: Float>(t10273: F, t558: F, t10240: F, t10243: F, t10246: F, t10250: F, t10254: F, t10258: F, t10263: F, t10265: F, t10269: F, t3004: F, t554: F, t557: F, t8208: F, t8211: F) -> (F, F) {
    let t10274 = t558 * t10273;
    let t10278 = -t8208 - t8211 - t10240 / 192.0 - t10243 / 96.0 - t554 * t557 * t10246 / 64.0 - t554 * t557 * t10250 / 32.0 - t554 * t3004 * t10254 / 16.0 - t554 * t557 * t10258 / 32.0 - t10263 / 144.0 - t554 * t557 * t10265 / 64.0 - t554 * t557 * t10269 / 64.0 - t554 * t557 * t10274 / 64.0;
    (t10274, t10278)
}
