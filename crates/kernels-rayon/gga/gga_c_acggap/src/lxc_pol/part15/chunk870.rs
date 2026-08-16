//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 870/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk870(t377: f64, t7779: f64, t606: f64, t7: f64, t7508: f64, t8: f64, t151: f64, t56: f64, t593: f64, t1994: f64, t1039: f64, t1997: f64, t3055: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30169 = t377 * t7779;
    let t30170 = t30169 * t606;
    let t30174 = t7508 * t7;
    let t30176 = 1.0_f64 / t8 / t30174;
    let t30179 = t151 * t593 * t30176 * t56;
    let t30180 = t30179 * t1994;
    let t30183 = t3055 * t1997 * t1039;
    (t30169, t30170, t30174, t30179, t30180, t30183)
}
