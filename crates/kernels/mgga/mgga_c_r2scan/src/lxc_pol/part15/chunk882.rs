//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 882/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk882<F: Float>(t2122: F, t2133: F, t2136: F, t2139: F, t2142: F, t2557: F, t2564: F, t2569: F, t2598: F, t2600: F, t6149: F, t6152: F, t6272: F, t7984: F, t7987: F, t7991: F, t7996: F, t8003: F, t8007: F, t8014: F, t8018: F, t8022: F, t8026: F, t8029: F, t8031: F, t8035: F, t8039: F) -> F {
    let t8042 = F::cast_from(0.86682217400542685632e-1_f64) * t7984 * t2136 + F::cast_from(0.2600466522016280569e0_f64) * t7987 * t2142 + F::cast_from(0.43341108700271342816e-1_f64) * t2133 * t7991 + F::cast_from(0.10975748638225852664e0_f64) * t2122 * t7996 + F::cast_from(0.86682217400542685632e-1_f64) * t6149 * t2564 + F::cast_from(0.86682217400542685632e-1_f64) * t2133 * t8003 + F::cast_from(0.13002332610081402845e0_f64) * t2139 * t8007 + F::cast_from(0.2600466522016280569e0_f64) * t6152 * t2569 + F::cast_from(0.10975748638225852664e0_f64) * t2557 * t8014 + F::cast_from(0.54878743191129263322e-1_f64) * t2557 * t8018 + F::cast_from(0.17336443480108537126e0_f64) * t8022 * t2600 + F::cast_from(0.54878743191129263322e-2_f64) * t6272 - F::cast_from(0.21341733463216935736e0_f64) * t8026 - F::cast_from(0.2600466522016280569e0_f64) * t8029 * t8031 + F::cast_from(0.26004665220162805689e0_f64) * t2598 * t8035 + F::cast_from(0.54878743191129263322e-1_f64) * t2122 * t8039;
    t8042
}
