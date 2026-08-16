//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 834/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk834(t2297: f64, t513: f64, t4262: f64, t2030: f64, t1755: f64, t7822: f64, t1761: f64, t1859: f64, t604: f64, t1181: f64, t7575: f64, t1089: f64, t1459: f64, t9563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9711 = t2297 * t513;
    let t9712 = t4262 * t9711;
    let t9713 = t2030 * t9712;
    let t9715 = t7822 * t1755;
    let t9717 = t7822 * t1761;
    let t9719 = t604 * t1859;
    let t9720 = t1181 * t9719;
    let t9721 = t7575 * t9720;
    let t9724 = t1089 * t1459 * t9563;
    (t9711, t9712, t9713, t9715, t9717, t9719, t9720, t9721, t9724)
}
