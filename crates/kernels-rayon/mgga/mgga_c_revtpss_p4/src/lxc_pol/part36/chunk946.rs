//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 946/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk946(t4279: f64, t5911: f64, t22604: f64, t108: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t22597: f64, t22600: f64, t22605: f64, t22608: f64, t22618: f64, t5902: f64, t5908: f64, t5912: f64, t97: f64) -> f64 {
    let t22621 = t4279 * t5911;
    let t22624 = -t22604;
    let t22625 = t108 * t22624;
    let t22628 = -10.0_f64 / 27.0_f64 * t97 * t22597 + 10.0_f64 / 3.0_f64 * t97 * t22600 + 5.0_f64 / 3.0_f64 * t97 * t22605 - 440.0_f64 / 27.0_f64 * t22608 * t109 + 200.0_f64 / 9.0_f64 * t5902 * t1510 - 50.0_f64 / 9.0_f64 * t1507 * t5908 - 25.0_f64 / 3.0_f64 * t1507 * t5912 - 10.0_f64 / 27.0_f64 * t105 * t22618 + 10.0_f64 / 3.0_f64 * t105 * t22621 + 5.0_f64 / 3.0_f64 * t105 * t22625;
    t22628
}
