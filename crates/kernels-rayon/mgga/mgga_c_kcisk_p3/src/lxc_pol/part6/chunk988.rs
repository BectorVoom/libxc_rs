//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 988/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk988(t2128: f64, t7927: f64, t4083: f64, t2119: f64, t7931: f64, t13632: f64, t13618: f64, t20292: f64, t26138: f64, t26150: f64, t26159: f64, t30288: f64, t30292: f64, t30296: f64, t30300: f64, t30303: f64, t30306: f64) -> (f64, f64, f64, f64, f64) {
    let t30318 = t7927 * t2128;
    let t30319 = t30318 * t4083;
    let t30326 = t7931 * t2119;
    let t30327 = t13632 * t30326;
    let t30339 = -t13618 - 4.0_f64 / 9.0_f64 * t20292 + 2.0_f64 / 9.0_f64 * t26138 - 2.0_f64 / 3.0_f64 * t26150 + t26159 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t30288 + 4.0_f64 / 3.0_f64 * t30292 - 2.0_f64 / 3.0_f64 * t30296 - 2.0_f64 * t30300 + 2.0_f64 * t30303 - t30306 / 3.0_f64;
    (t30318, t30319, t30326, t30327, t30339)
}
