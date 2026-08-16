//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 423/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk423(t118: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t508: f64, t569: f64, t651: f64, t3: f64, param_d: f64) -> (f64, f64, f64) {
    let t2110 = -t118 * t2089 + t2014 * t2108 - t2052 * t508 - 2.0_f64 * t2056 * t651 + t2093 * t569;
    let t2111 = t3 * t2110;
    let t2113 = param_d * t2110;
    (t2110, t2111, t2113)
}
