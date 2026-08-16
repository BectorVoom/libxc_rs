//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 197/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk197(t41: f64, t604: f64, t525: f64, t642: f64, t79: f64, t20: f64, t718: f64) -> (f64, f64, f64, f64, f64) {
    let t773 = t604 * t41;
    let t776 = 10.0_f64 / 9.0_f64 * t525 * t773 * t642;
    let t777 = t776 < -0.66725e-1_f64;
    let t779 = piecewise3(t777, 0.0_f64, 0.66725e-1_f64 + t776);
    let t780 = t79 * t779;
    let t781 = t718 * t20;
    let t782 = t780 * t781;
    (t773, t780, t781, t782, t776)
}
