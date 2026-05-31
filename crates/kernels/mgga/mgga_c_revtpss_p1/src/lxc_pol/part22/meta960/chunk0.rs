//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3221/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221<F: Float>(t18299: F, t750: F, t49911: F, t4537: F, t18298: F, t705: F, t707: F, t14749: F, t14767: F, t198: F, t207: F, t2411: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t4541: F, t4546: F) -> (F, F, F, F) {
    let t61114 = t18299 * t750;
    let t61115 = F::cast_from(2.0_f64) * t61114;
    let t61116 = F::cast_from(48.0_f64) * t49911;
    let t61117 = t4537 * t4537;
    let t61122 = t705 * t18298;
    let t61124 = F::cast_from(8.0_f64) * t61122 * t707;
    let t61125 = -F::cast_from(2.0_f64) * t198 * t207 * t2411 * t61117 + F::cast_from(24.0_f64) * t14749 * t4541 * t4546 + F::cast_from(12.0_f64) * t14767 * t4541 * t4546 - t39483 + t39520 - t39528 + t39531 + t39534 + t39537 + t61115 + t61116 + t61124;
    (t61115, t61116, t61124, t61125)
}
