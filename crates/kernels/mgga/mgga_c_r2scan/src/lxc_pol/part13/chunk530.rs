//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 530/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk530<F: Float>(t2590: F, t2591: F, t2124: F, t1601: F, t1608: F, t1612: F, t1619: F, t1622: F, t1635: F, t2122: F, t2133: F, t2139: F, t2547: F, t2553: F, t2557: F, t2559: F, t2564: F, t2569: F, t2575: F, t2579: F, t2582: F, t2584: F) -> (F, F) {
    let t2592 = t2590 * t2591;
    let t2593 = t2124 * t2592;
    let t2596 = 0.64025200389650807209e-1 * t1601 + 0.54878743191129263322e-1 * t2122 * t2547 + 0.54878743191129263322e-1 * t2122 * t2553 - 0.27439371595564631661e-1 * t2557 * t2559 + 0.43341108700271342816e-1 * t2133 * t2564 + 0.13002332610081402845e0 * t2139 * t2569 + 0.43341108700271342816e-1 * t2133 * t2575 + 0.13002332610081402845e0 * t2139 * t2579 - 0.43341108700271342816e-1 * t2582 * t2584 + 0.54878743191129263322e-2 * t1608 + 0.58218257753910989057e-2 * t1612 - t1619 - t1622 + 0.11557628986739024751e0 * t1635 + 0.54878743191129263322e-1 * t2557 * t2593;
    (t2593, t2596)
}
