//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 496/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk496(t2590: f64, t2591: f64, t2124: f64, t1601: f64, t1608: f64, t1612: f64, t1619: f64, t1622: f64, t1635: f64, t2122: f64, t2133: f64, t2139: f64, t2547: f64, t2553: f64, t2557: f64, t2559: f64, t2564: f64, t2569: f64, t2575: f64, t2579: f64, t2582: f64, t2584: f64) -> (f64, f64) {
    let t2592 = t2590 * t2591;
    let t2593 = t2124 * t2592;
    let t2596 = 0.64025200389650807209e-1_f64 * t1601 + 0.54878743191129263322e-1_f64 * t2122 * t2547 + 0.54878743191129263322e-1_f64 * t2122 * t2553 - 0.27439371595564631661e-1_f64 * t2557 * t2559 + 0.43341108700271342816e-1_f64 * t2133 * t2564 + 0.13002332610081402845e0_f64 * t2139 * t2569 + 0.43341108700271342816e-1_f64 * t2133 * t2575 + 0.13002332610081402845e0_f64 * t2139 * t2579 - 0.43341108700271342816e-1_f64 * t2582 * t2584 + 0.54878743191129263322e-2_f64 * t1608 + 0.58218257753910989057e-2_f64 * t1612 - t1619 - t1622 + 0.11557628986739024751e0_f64 * t1635 + 0.54878743191129263322e-1_f64 * t2557 * t2593;
    (t2593, t2596)
}
