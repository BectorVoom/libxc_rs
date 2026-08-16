//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1366/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1366(t65616: f64, t65624: f64, t65628: f64, t62390: f64, t65604: f64, t65608: f64, t65611: f64, t65614: f64, t65618: f64, t65620: f64, t65622: f64, t65626: f64, t65630: f64) -> f64 {
    let t67169 = 35.0_f64 / 144.0_f64 * t65616;
    let t67173 = 119.0_f64 / 3456.0_f64 * t65624;
    let t67175 = 7.0_f64 / 576.0_f64 * t65628;
    let t67177 = t65604 / 96.0_f64 - t65608 / 128.0_f64 + t65611 / 4.0_f64 + t65614 / 8.0_f64 - t67169 + 5.0_f64 / 96.0_f64 * t65618 + 5.0_f64 / 192.0_f64 * t65620 - t65622 / 768.0_f64 - t62390 - t67173 - 5.0_f64 / 32.0_f64 * t65626 + t67175 - t65630 / 768.0_f64;
    t67177
}
