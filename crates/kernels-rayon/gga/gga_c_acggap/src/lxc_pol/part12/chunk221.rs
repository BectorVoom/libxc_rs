//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 221/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk221(t219: f64, t771: f64, t201: f64, t199: f64, t13: f64, t30: f64, t761: f64, t132: f64, t265: f64, t264: f64, t80: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t772 = t771 * t219;
    let t773 = t201 * t772;
    let t774 = 1.0_f64 * t773;
    let t775 = t199 * t199;
    let t776 = 1.0_f64 / t775;
    let t777 = t13 * t776;
    let t778 = t30 * t30;
    let t779 = 1.0_f64 / t778;
    let t780 = t761 * t779;
    let t781 = t777 * t780;
    let t782 = 0.16081979498692535067e2_f64 * t781;
    let t786 = t132 * t265;
    let t790 = t264 * t80;
    let t791 = 1.0_f64 / t790;
    let t792 = t75 * t791;
    (t772, t774, t775, t776, t777, t778, t779, t780, t782, t786, t791, t792)
}
