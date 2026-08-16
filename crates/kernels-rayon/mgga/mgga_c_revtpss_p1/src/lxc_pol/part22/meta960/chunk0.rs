//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3221/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221(t18299: f64, t750: f64, t49911: f64, t4537: f64, t18298: f64, t705: f64, t707: f64, t14749: f64, t14767: f64, t198: f64, t207: f64, t2411: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t4541: f64, t4546: f64) -> (f64, f64, f64, f64) {
    let t61114 = t18299 * t750;
    let t61115 = 2.0_f64 * t61114;
    let t61116 = 48.0_f64 * t49911;
    let t61117 = t4537 * t4537;
    let t61122 = t705 * t18298;
    let t61124 = 8.0_f64 * t61122 * t707;
    let t61125 = -2.0_f64 * t198 * t207 * t2411 * t61117 + 24.0_f64 * t14749 * t4541 * t4546 + 12.0_f64 * t14767 * t4541 * t4546 - t39483 + t39520 - t39528 + t39531 + t39534 + t39537 + t61115 + t61116 + t61124;
    (t61115, t61116, t61124, t61125)
}
