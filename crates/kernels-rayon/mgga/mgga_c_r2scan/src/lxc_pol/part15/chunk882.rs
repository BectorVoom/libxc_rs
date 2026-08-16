//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 882/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk882(t2122: f64, t2133: f64, t2136: f64, t2139: f64, t2142: f64, t2557: f64, t2564: f64, t2569: f64, t2598: f64, t2600: f64, t6149: f64, t6152: f64, t6272: f64, t7984: f64, t7987: f64, t7991: f64, t7996: f64, t8003: f64, t8007: f64, t8014: f64, t8018: f64, t8022: f64, t8026: f64, t8029: f64, t8031: f64, t8035: f64, t8039: f64) -> f64 {
    let t8042 = 0.86682217400542685632e-1_f64 * t7984 * t2136 + 0.2600466522016280569e0_f64 * t7987 * t2142 + 0.43341108700271342816e-1_f64 * t2133 * t7991 + 0.10975748638225852664e0_f64 * t2122 * t7996 + 0.86682217400542685632e-1_f64 * t6149 * t2564 + 0.86682217400542685632e-1_f64 * t2133 * t8003 + 0.13002332610081402845e0_f64 * t2139 * t8007 + 0.2600466522016280569e0_f64 * t6152 * t2569 + 0.10975748638225852664e0_f64 * t2557 * t8014 + 0.54878743191129263322e-1_f64 * t2557 * t8018 + 0.17336443480108537126e0_f64 * t8022 * t2600 + 0.54878743191129263322e-2_f64 * t6272 - 0.21341733463216935736e0_f64 * t8026 - 0.2600466522016280569e0_f64 * t8029 * t8031 + 0.26004665220162805689e0_f64 * t2598 * t8035 + 0.54878743191129263322e-1_f64 * t2122 * t8039;
    t8042
}
