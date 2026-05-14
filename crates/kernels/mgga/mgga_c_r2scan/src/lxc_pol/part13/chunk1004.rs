//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1004/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1004<F: Float>(t10769: F, t39409: F, t10772: F, t3308: F, t7978: F, t8006: F, t39385: F, t39387: F, t39390: F, t39393: F, t39396: F, t39397: F, t39401: F, t39404: F, t39406: F, t2547: F, t37764: F) -> (F, F) {
    let t39410 = t39409 * t10769;
    let t39411 = 0.47609969197673950972e-2 * t39410;
    let t39413 = t10772 * t3308 * t7978;
    let t39416 = t10772 * t3308 * t8006;
    let t39418 = -0.43341108700271342816e-1 * t39385 - 0.86682217400542685632e-1 * t39387 + 0.86682217400542685632e-1 * t39390 + 0.2600466522016280569e0 * t39393 + t39396 - 0.27439371595564631661e-1 * t39397 - t39401 - t39404 - 0.43341108700271342816e-1 * t39406 + t39411 + 0.2600466522016280569e0 * t39413 + 0.13002332610081402845e0 * t39416;
    let t39420 = t37764 * t2547;
    (t39418, t39420)
}
