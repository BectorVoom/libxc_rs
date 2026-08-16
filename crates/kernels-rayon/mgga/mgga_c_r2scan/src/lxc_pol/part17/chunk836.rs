//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 836/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk836(t2124: f64, t495: f64, t8842: f64, t3052: f64, t537: f64, t2551: f64, t2122: f64, t2133: f64, t2139: f64, t2557: f64, t2569: f64, t6132: f64, t6293: f64, t7388: f64, t7393: f64, t7395: f64, t7397: f64, t7399: f64, t7401: f64, t7405: f64, t7408: f64, t7987: f64, t8822: f64, t8827: f64, t8834: f64, t8839: f64) -> (f64, f64, f64, f64, f64) {
    let t8844 = t2124 * t8842 * t495;
    let t8847 = t537 * t3052;
    let t8849 = t2124 * t8847 * t2551;
    let t8853 = t2124 * t8847 * t495;
    let t8858 = -0.86682217400542685632e-1_f64 * t6132 * t8822 + 0.43341108700271342816e-1_f64 * t2133 * t8827 + 0.2600466522016280569e0_f64 * t7987 * t2569 + 0.13002332610081402845e0_f64 * t2139 * t8834 + 0.54878743191129263322e-1_f64 * t2122 * t8839 - 0.16463622957338778997e0_f64 * t6293 * t8844 + 0.54878743191129263322e-1_f64 * t2122 * t8849 - 0.27439371595564631661e-1_f64 * t2557 * t8853 - 0.84755945902752848174e0_f64 * t7388 + t7393 + t7395 + t7397 + t7399 + t7401 - t7405 - 0.32927245914677557992e-1_f64 * t7408;
    (t8844, t8847, t8849, t8853, t8858)
}
