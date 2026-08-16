//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 834/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk834(t142: f64, t9704: f64, t2060: f64, t2297: f64, t513: f64, t4262: f64, t2030: f64, t1755: f64, t7822: f64, t1761: f64, t1859: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9705 = t142 * t9704;
    let t9706 = t2060 * t9705;
    let t9711 = t2297 * t513;
    let t9712 = t4262 * t9711;
    let t9713 = t2030 * t9712;
    let t9715 = t7822 * t1755;
    let t9717 = t7822 * t1761;
    let t9719 = t604 * t1859;
    (t9705, t9706, t9711, t9712, t9713, t9715, t9717, t9719)
}
