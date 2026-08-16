//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3436/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3436(t15573: f64, t4719: f64, t11524: f64, t19133: f64, t981: f64, t15526: f64, t19134: f64, t3022: f64, t15266: f64, t52894: f64, t63597: f64, t19021: f64, t3011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64493 = 0.2077903092681775651e3_f64 * t4719 * t15573;
    let t64496 = 0.10389515463408878255e3_f64 * t981 * t19133 * t11524;
    let t64498 = 0.69263436422725855034e2_f64 * t4719 * t15526;
    let t64500 = 0.20779030926817756511e3_f64 * t3022 * t19134;
    let t64503 = 0.41016075432865626631e4_f64 * t52894 * t15266 * t63597;
    let t64504 = t3011 * t19021;
    (t64493, t64496, t64498, t64500, t64503, t64504)
}
