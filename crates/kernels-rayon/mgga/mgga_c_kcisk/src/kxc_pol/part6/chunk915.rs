//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 915/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk915(t645: f64, t29188: f64, t29259: f64, t67: f64, t2441: f64, t8780: f64, t11153: f64, t1755: f64, t28314: f64, t2436: f64, t2442: f64, t28800: f64, t340: f64, t6141: f64, t639: f64, t642: f64, t7196: f64, t8773: f64, t8781: f64, t8787: f64) -> (f64, f64, f64, f64) {
    let t646 = t645 < -0.66725e-1_f64;
    let t29261 = t67 * (t29188 + t29259);
    let t29274 = t8780 * t2441;
    let t29275 = t11153 * t29274;
    let t29282 = t1755 * t28314;
    let t29287 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t29261 * t642 - 10.0_f64 / 9.0_f64 * t340 * t8773 * t2442 + 40.0_f64 / 27.0_f64 * t340 * t2436 * t8781 - 10.0_f64 / 9.0_f64 * t340 * t2436 * t8787 - 280.0_f64 / 243.0_f64 * t340 * t639 * t29275 + 40.0_f64 / 27.0_f64 * t6141 * t7196 * t28800 - 10.0_f64 / 27.0_f64 * t340 * t639 * t29282);
    (t29274, t29275, t29282, t29287)
}
