//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 271/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk271(t145: f64, t717: f64, t185: f64, t164: f64, t159: f64, t688: f64, t690: f64, t694: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t718 = t145 * t717;
    let t719 = t718 * t185;
    let t723 = t164 * t164;
    let t724 = 1.0_f64 / t723;
    let t725 = t159 * t724;
    let t730 = -0.1176575e1_f64 * t688 - 0.516475e0_f64 * t690 - 0.2103875e0_f64 * t694 - 0.104195e0_f64 * t699;
    (t718, t719, t723, t724, t725, t730)
}
