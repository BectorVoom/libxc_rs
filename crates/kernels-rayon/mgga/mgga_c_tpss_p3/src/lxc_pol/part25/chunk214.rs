//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 214/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk214(t45: f64, t57: f64, t190: f64, t581: f64, t681: f64, t78: f64, t81: f64, t150: f64, t169: f64, t164: f64, t662: f64, t664: f64, t668: f64, t673: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t682 = t190 * t581;
    let t684 = 4.0_f64 * t681 * t682;
    let t687 = piecewise3(t151, 0.0_f64, 4.0_f64 / 3.0_f64 * t78 * t581);
    let t690 = piecewise3(t155, 0.0_f64, -4.0_f64 / 3.0_f64 * t81 * t581);
    let t691 = t687 + t690;
    let t692 = t150 * t691;
    let t693 = t692 * t190;
    let t697 = t169 * t169;
    let t698 = 1.0_f64 / t697;
    let t699 = t164 * t698;
    let t704 = -0.1176575e1_f64 * t662 - 0.516475e0_f64 * t664 - 0.2103875e0_f64 * t668 - 0.104195e0_f64 * t673;
    (t682, t684, t691, t692, t693, t697, t698, t699, t704)
}
