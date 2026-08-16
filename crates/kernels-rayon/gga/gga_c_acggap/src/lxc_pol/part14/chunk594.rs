//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 594/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk594(t1713: f64, t360: f64, t1426: f64, t368: f64, t1782: f64, t322: f64, t1459: f64, t398: f64, t384: f64, t301: f64, t1089: f64, t1817: f64, t3329: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5559 = t1713 * t360;
    let t5561 = t1426 * t368 * t5559;
    let t5567 = t1782 * t322;
    let t5569 = t398 * t1459 * t5567;
    let t5570 = t384 * t5569;
    let t5572 = t1782 * t301;
    let t5574 = t1089 * t1459 * t5572;
    let t5577 = t3329 * t1817;
    (t5559, t5561, t5567, t5569, t5570, t5572, t5574, t5577)
}
