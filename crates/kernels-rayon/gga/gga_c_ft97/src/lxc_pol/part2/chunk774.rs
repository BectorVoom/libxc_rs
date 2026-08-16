//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 774/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk774(t2253: f64, t3642: f64, t11034: f64, t3613: f64, t1736: f64, t179: f64, t11008: f64, t12099: f64, t12102: f64, t12104: f64, t12108: f64, t12113: f64, t12119: f64, t12123: f64, t12128: f64, t2265: f64, t631: f64, t8641: f64, t8643: f64, t8645: f64, t8647: f64, t8676: f64, t8678: f64, t8714: f64, t8718: f64, t8719: f64) -> f64 {
    let t12132 = 2.0_f64 * t2253 * t3642;
    let t12134 = t3613 * t11034;
    let t12137 = t1736 * t179;
    let t12138 = t12137 * t11008;
    let t12141 = 10.0_f64 / 27.0_f64 * t8641 - t8643 / 9.0_f64 - t8645 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t2265 * t12099 + t2265 * t12102 - t2265 * t12104 / 3.0_f64 + t2265 * t12108 + t8647 - t8714 / 3.0_f64 + 10.0_f64 / 9.0_f64 * t8719 + 2.0_f64 * t2265 * t12113 + 4.0_f64 / 3.0_f64 * t2265 * t12119 - 2.0_f64 / 9.0_f64 * t2265 * t12123 + 4.0_f64 / 9.0_f64 * t8676 + t8718 - 3.0_f64 / 2.0_f64 * t631 * t12128 + t12132 + 2.0_f64 / 9.0_f64 * t8678 + t2265 * t12134 / 18.0_f64 + 2.0_f64 / 27.0_f64 * t2265 * t12138;
    t12141
}
