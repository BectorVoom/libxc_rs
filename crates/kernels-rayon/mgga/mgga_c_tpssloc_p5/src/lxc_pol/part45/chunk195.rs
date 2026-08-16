//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 195/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk195(t40: f64, t52: f64, t31: f64, t32: f64, t152: f64, t185: f64, t607: f64, t73: f64, t76: f64, t145: f64, t164: f64, t159: f64, t688: f64, t690: f64, t694: f64, t699: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t706 = t32 * t31;
    let t707 = t706 * t152;
    let t708 = t185 * t607;
    let t710 = 4.0_f64 * t707 * t708;
    let t713 = piecewise3(t146, 0.0_f64, 4.0_f64 / 3.0_f64 * t73 * t607);
    let t716 = piecewise3(t150, 0.0_f64, -4.0_f64 / 3.0_f64 * t76 * t607);
    let t717 = t713 + t716;
    let t718 = t145 * t717;
    let t719 = t718 * t185;
    let t723 = t164 * t164;
    let t724 = 1.0_f64 / t723;
    let t725 = t159 * t724;
    let t730 = -0.1176575e1_f64 * t688 - 0.516475e0_f64 * t690 - 0.2103875e0_f64 * t694 - 0.104195e0_f64 * t699;
    (t706, t707, t708, t710, t717, t718, t719, t723, t724, t725, t730)
}
