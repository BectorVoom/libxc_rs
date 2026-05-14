//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1002/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1002<F: Float>(t39352: F, t39355: F, t39358: F, t39362: F, t39364: F, t39367: F, t39370: F, t39373: F, t39376: F, t39379: F, t39381: F, t3308: F, t574: F, t7940: F, t11797: F, t1584: F) -> (F, F, F) {
    let t39383 = -0.16463622957338778997e0 * t39352 - 0.14282990759302185291e-1 * t39355 - 0.57131963037208741166e-1 * t39358 - t39362 + 0.43341108700271342816e-1 * t39364 + 0.13002332610081402845e0 * t39367 - 0.86682217400542685632e-1 * t39370 - 0.86682217400542685632e-1 * t39373 + 0.86682217400542685632e-1 * t39376 + 0.2600466522016280569e0 * t39379 + 0.86682217400542685632e-1 * t39381;
    let t39385 = t574 * t3308 * t7940;
    let t39387 = t1584 * t11797;
    (t39383, t39385, t39387)
}
