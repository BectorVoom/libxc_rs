//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 850/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk850(t16679: f64, t13100: f64, t13101: f64, t16668: f64, t16673: f64, t16677: f64, t16684: f64, t16689: f64, t16692: f64, t16696: f64, t16699: f64, t12359: f64, t12362: f64, t12571: f64, t13102: f64, t13108: f64, t13117: f64, t13120: f64, t16706: f64, t9166: f64, t9369: f64, t9371: f64) -> (f64, f64) {
    let t17214 = 2.0_f64 / 9.0_f64 * t16679;
    let t17220 = -4.0_f64 / 3.0_f64 * t16668 - 4.0_f64 / 3.0_f64 * t16673 + 4.0_f64 / 9.0_f64 * t16677 - t17214 + t16684 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t16689 + 8.0_f64 / 3.0_f64 * t16692 + t16696 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t16699 - t13100 - t13101;
    let t17225 = t13102 - t13108 - t9369 - t9371 - t13117 + 4.0_f64 / 9.0_f64 * t12359 - 8.0_f64 / 27.0_f64 * t12362 - t9166 + t13120 - 8.0_f64 / 9.0_f64 * t12571 - 2.0_f64 / 9.0_f64 * t16706;
    (t17220, t17225)
}
