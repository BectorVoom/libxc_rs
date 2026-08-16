//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 199/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk199(t160: f64, t405: f64, t164: f64, t162: f64, t271: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t601 = t160 * t405;
    let t602 = t601 * t164;
    let t603 = t162 * t271;
    let t604 = 1.0_f64 / t603;
    let t605 = t161 * t604;
    let t607 = -12.0_f64 * t602 + 12.0_f64 * t605;
    (t601, t602, t603, t604, t605, t607)
}
