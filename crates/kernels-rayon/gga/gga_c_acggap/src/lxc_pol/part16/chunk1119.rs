//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1119/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1119(t1844: f64, t322: f64, t1165: f64, t604: f64, t7346: f64, t1181: f64, t2068: f64, t39164: f64, t2016: f64, t9630: f64, t1327: f64, t507: f64, t8888: f64) -> (f64, f64, f64, f64, f64) {
    let t39499 = t1844 * t322;
    let t39502 = t7346 * t1165 * t604 * t39499;
    let t39506 = t2068 * t1181 * t604 * t39164;
    let t39508 = t2016 * t9630;
    let t39511 = t8888 * t507 * t1327;
    (t39499, t39502, t39506, t39508, t39511)
}
