//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 592/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk592(t1938: f64, t857: f64, t1782: f64, t360: f64, t3300: f64, t398: f64, t372: f64, t5011: f64, t1524: f64, t513: f64, t1459: f64, t1008: f64, t1851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5525 = t857 * t1938;
    let t5527 = t1782 * t360;
    let t5529 = t398 * t3300 * t5527;
    let t5532 = t1782 * t372;
    let t5534 = t398 * t5011 * t5532;
    let t5537 = t513 * t1524;
    let t5539 = t398 * t1459 * t5537;
    let t5542 = t1008 * t1851;
    (t5525, t5527, t5529, t5532, t5534, t5537, t5539, t5542)
}
