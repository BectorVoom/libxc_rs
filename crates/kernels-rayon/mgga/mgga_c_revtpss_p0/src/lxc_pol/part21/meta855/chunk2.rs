//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3237/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3237(t1300: f64, t198: f64, t336: f64, t56390: f64, t56484: f64, t56534: f64, t56593: f64, t56642: f64, t56687: f64, t57794: f64, t57799: f64, t57802: f64, t57805: f64, t57808: f64, t57810: f64, t57812: f64, t57814: f64, t57816: f64, t57820: f64, t60068: f64, t60117: f64) -> f64 {
    let t60124 = t198 * t336 * (t56390 + t56484 + t56534 + t56593 + t56642 + t56687 + t60068 + t60117) * t1300 - t57794 + t57799 - t57802 - t57805 - t57808 - t57810 - t57812 - t57814 + t57816 - t57820;
    t60124
}
