//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 895/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk895<F: Float>(t551: F, t552: F, t6364: F, t2122: F, t2133: F, t2196: F, t2223: F, t2557: F, t535: F, t6278: F, t6283: F, t6288: F, t6293: F, t6296: F, t6300: F, t6304: F, t6310: F, t6324: F, t6333: F, t6336: F, t6340: F, t6346: F, t6349: F, t6352: F, t6355: F, t6362: F) -> (F, F) {
    let t6366 = t551 * t552 * t6364;
    let t6369 = -0.82318114786693894983e-1 * t2557 * t6278 + 0.16463622957338778996e0 * t2122 * t6283 + 0.13002332610081402845e0 * t2133 * t6288 - 0.49390868872016336989e0 * t6293 * t6296 + 0.16463622957338778996e0 * t2122 * t6300 - 0.69345773920434148506e0 * t6304 - t6310 + t6324 + t6333 + 0.49390868872016336991e0 * t2223 * t6336 + 0.15602799132097683414e1 * t2196 * t6340 - 0.12713391885412927226e1 * t6346 - 0.27439371595564631661e-1 * t535 * t6349 + 0.19207560116895242163e0 * t6352 - 0.27439371595564631661e-1 * t535 * t6355 - 0.2600466522016280569e0 * t6362 * t6366;
    (t6366, t6369)
}
