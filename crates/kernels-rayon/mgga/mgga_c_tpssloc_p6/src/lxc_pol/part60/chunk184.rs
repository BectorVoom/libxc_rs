//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 184/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk184(t31: f64, t32: f64, t152: f64, t164: f64, t159: f64, t688: f64, t690: f64, t694: f64, t699: f64, t167: f64, t177: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t706 = t32 * t31;
    let t707 = t706 * t152;
    let t723 = t164 * t164;
    let t724 = 1.0_f64 / t723;
    let t725 = t159 * t724;
    let t730 = -0.1176575e1_f64 * t688 - 0.516475e0_f64 * t690 - 0.2103875e0_f64 * t694 - 0.104195e0_f64 * t699;
    let t731 = 1.0_f64 / t167;
    let t732 = t730 * t731;
    let t738 = t177 * t177;
    let t739 = 1.0_f64 / t738;
    let t740 = t172 * t739;
    (t706, t707, t723, t724, t725, t730, t731, t732, t738, t739, t740)
}
