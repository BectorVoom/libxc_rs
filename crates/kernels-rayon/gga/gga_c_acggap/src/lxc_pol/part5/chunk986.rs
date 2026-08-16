//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 986/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk986(t13695: f64, t1480: f64, t133: f64, t168: f64, t3111: f64, t4807: f64, t1060: f64, t355: f64, t4099: f64, t721: f64, t145: f64, t4875: f64) -> (f64, f64, f64, f64, f64) {
    let t16294 = t13695 * t1480;
    let t16296 = t133 * t168;
    let t16300 = t3111 * t4807;
    let t16304 = t1060 * t355 * t4099 * t721;
    let t16314 = t4875 * t145;
    (t16294, t16296, t16300, t16304, t16314)
}
