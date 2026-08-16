//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1024/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1024(t36039: f64, t31142: f64, t8810: f64, t7440: f64, t8803: f64, t1181: f64, t4623: f64, t604: f64, t7426: f64, t30090: f64, t8897: f64, t31362: f64, t8903: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36040 = 7.0_f64 / 24.0_f64 * t36039;
    let t36041 = t31142 * t8810;
    let t36042 = 7.0_f64 / 72.0_f64 * t36041;
    let t36065 = t7440 * t8803;
    let t36066 = 11.0_f64 / 288.0_f64 * t36065;
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36082 = 0.62896184579208304136e-3_f64 * t36081;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    (t36040, t36042, t36066, t36082, t36083, t36085)
}
