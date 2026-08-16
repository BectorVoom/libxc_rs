//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 502/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk502(t2625: f64, t506: f64, t529: f64, t2531: f64, t538: f64, t560: f64, t938: f64, t551: f64, t552: f64, t1584: f64, t1592: f64, t2083: f64, t2088: f64, t2095: f64, t2108: f64, t2119: f64, t2184: f64, t2196: f64, t2223: f64, t2598: f64, t2600: f64, t2606: f64, t2610: f64, t2614: f64, t2617: f64, t2621: f64, t535: f64, t948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2626 = t506 * t2625;
    let t2627 = t529 * t2626;
    let t2630 = t538 * t2531;
    let t2631 = t529 * t2630;
    let t2634 = t938 * t560;
    let t2636 = t551 * t552 * t2634;
    let t2640 = t551 * t552 * t2625;
    let t2643 = 0.86682217400542685632e-1_f64 * t2598 * t2600 - 0.11557628986739024751e0_f64 * t2083 + t2088 + t2095 + t2108 + t2119 - 0.48787202696913915093e-2_f64 * t2606 - 0.58218257753910989057e-2_f64 * t2610 + 0.13002332610081402845e0_f64 * t1592 * t2614 + 0.64025200389650807209e-1_f64 * t2617 + 0.11557628986739024751e0_f64 * t2621 - 0.43341108700271342816e-1_f64 * t1584 * t948 + 0.16463622957338778997e0_f64 * t2223 * t2627 - 0.27439371595564631661e-1_f64 * t535 * t2631 + 0.86682217400542685632e-1_f64 * t2184 * t2636 + 0.5200933044032561138e0_f64 * t2196 * t2640;
    (t2626, t2627, t2630, t2631, t2634, t2636, t2640, t2643)
}
