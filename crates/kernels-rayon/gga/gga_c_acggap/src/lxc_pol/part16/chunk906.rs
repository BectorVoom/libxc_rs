//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 906/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk906(t128: f64, t576: f64, t7475: f64, t1108: f64, t7736: f64, t7770: f64, t7799: f64, t1170: f64, t31114: f64) -> (f64, f64, f64, f64) {
    let t31146 = t576 * t7475 * t128;
    let t31160 = t7736 * t1108;
    let t31168 = t7799 * t7770;
    let t31195 = t1170 * t31114;
    (t31146, t31160, t31168, t31195)
}
