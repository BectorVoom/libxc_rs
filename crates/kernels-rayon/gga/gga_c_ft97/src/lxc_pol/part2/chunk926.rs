//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 926/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk926(t14332: f64, t14352: f64, t661: f64, t2330: f64, t3826: f64, t1136: f64, t9511: f64, t1137: f64, t1173: f64, t14013: f64, t14037: f64, t2331: f64, t2465: f64, t2617: f64, t263: f64, t3683: f64, t3827: f64, t4003: f64, t719: f64, t771: f64) -> f64 {
    let t14353 = t14332 + t14352;
    let t14354 = t661 * t14353;
    let t14358 = t2330 * t3826;
    let t14361 = t9511 * t1136;
    let t14365 = -t1137 * t2617 - t1173 * t2331 - t1173 * t2465 - t14354 * t263 - 2.0_f64 * t14358 * t263 - t14361 * t263 - 2.0_f64 * t3683 * t771 - 2.0_f64 * t3827 * t771 - 2.0_f64 * t4003 * t719 - 2.0_f64 * t14013 - 2.0_f64 * t14037;
    t14365
}
