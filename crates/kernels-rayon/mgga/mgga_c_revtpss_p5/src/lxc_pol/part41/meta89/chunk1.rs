//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 518/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk518(t114: f64, t101: f64, t2174: f64, t69: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t2175 = t2174 * t101;
    let t2178 = piecewise3(t115, 0.0_f64, -t69 * t2175 / 8.0_f64);
    (t2175, t2178)
}
