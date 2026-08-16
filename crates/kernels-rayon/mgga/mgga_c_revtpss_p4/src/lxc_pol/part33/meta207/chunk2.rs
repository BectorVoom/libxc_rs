//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 963/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk963(t1832: f64, t3801: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t5062: f64, t5065: f64, t5067: f64, t5070: f64, t5107: f64, t5111: f64, t5189: f64, t5191: f64, t5194: f64, t5196: f64, t5200: f64, t5204: f64, t5209: f64, t5501: f64) -> (f64, f64) {
    let t5505 = t1832 * t3801;
    let t5508 = t1300 * t198 * t336 * t5501 - t1298 * t5023 * t5505 - t5062 + t5065 + t5067 - t5070 + t5107 + t5111 + t5189 + t5191 - t5194 - t5196 + t5200 - t5204 - t5209;
    (t5505, t5508)
}
