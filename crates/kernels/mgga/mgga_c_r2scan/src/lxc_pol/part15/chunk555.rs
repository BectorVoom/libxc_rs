//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 555/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk555<F: Float>(t2634: F, t551: F, t552: F, t2625: F, t1584: F, t1592: F, t2083: F, t2088: F, t2095: F, t2108: F, t2119: F, t2184: F, t2196: F, t2223: F, t2598: F, t2600: F, t2606: F, t2610: F, t2614: F, t2617: F, t2621: F, t2627: F, t2631: F, t535: F, t948: F) -> (F, F, F) {
    let t2636 = t551 * t552 * t2634;
    let t2640 = t551 * t552 * t2625;
    let t2643 = F::cast_from(0.86682217400542685632e-1_f64) * t2598 * t2600 - F::cast_from(0.11557628986739024751e0_f64) * t2083 + t2088 + t2095 + t2108 + t2119 - F::cast_from(0.48787202696913915093e-2_f64) * t2606 - F::cast_from(0.58218257753910989057e-2_f64) * t2610 + F::cast_from(0.13002332610081402845e0_f64) * t1592 * t2614 + F::cast_from(0.64025200389650807209e-1_f64) * t2617 + F::cast_from(0.11557628986739024751e0_f64) * t2621 - F::cast_from(0.43341108700271342816e-1_f64) * t1584 * t948 + F::cast_from(0.16463622957338778997e0_f64) * t2223 * t2627 - F::cast_from(0.27439371595564631661e-1_f64) * t535 * t2631 + F::cast_from(0.86682217400542685632e-1_f64) * t2184 * t2636 + F::cast_from(0.5200933044032561138e0_f64) * t2196 * t2640;
    (t2636, t2640, t2643)
}
