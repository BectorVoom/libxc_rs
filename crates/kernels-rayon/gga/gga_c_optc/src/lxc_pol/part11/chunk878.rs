//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 878/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk878(t16708: f64, t2520: f64, t1333: f64, t4786: f64, t7557: f64, t10188: f64, t13699: f64, t13701: f64, t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16650: f64, t7524: f64) -> (f64, f64, f64, f64) {
    let t16709 = t16708 * t2520;
    let t16715 = t4786 * t1333;
    let t16716 = t7557 * t16715;
    let t16729 = -t7524 - 4.0_f64 / 9.0_f64 * t10188 + 2.0_f64 / 9.0_f64 * t13699 - 2.0_f64 / 3.0_f64 * t13701 + t13703 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t16630 + 4.0_f64 / 3.0_f64 * t16634 - 2.0_f64 / 3.0_f64 * t16638 - 2.0_f64 * t16642 + 2.0_f64 * t16646 - t16650 / 3.0_f64;
    (t16709, t16715, t16716, t16729)
}
