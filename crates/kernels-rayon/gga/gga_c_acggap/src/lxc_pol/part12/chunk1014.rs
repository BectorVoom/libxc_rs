//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1014/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1014(t5138: f64, t8511: f64, t5143: f64, t31362: f64, t8783: f64, t1165: f64, t20595: f64, t604: f64, t7337: f64, t1426: f64, t1579: f64, t2085: f64, t598: f64) -> (f64, f64, f64, f64, f64) {
    let t34074 = t8511 * t5138;
    let t34078 = t8511 * t5143;
    let t34081 = t31362 * t8783;
    let t34085 = t7337 * t1165 * t604 * t20595;
    let t34089 = t598 * t1426 * t1579 * t2085;
    (t34074, t34078, t34081, t34085, t34089)
}
