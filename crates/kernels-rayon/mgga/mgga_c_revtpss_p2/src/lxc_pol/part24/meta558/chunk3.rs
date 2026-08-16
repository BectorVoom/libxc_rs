//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1672/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1672(t11387: f64, t41588: f64, t88031: f64, t41592: f64, t77499: f64, t77505: f64, t77507: f64, t77509: f64, t77663: f64, t77667: f64, t88089: f64, t88097: f64, t88144: f64, t88147: f64, t88150: f64, t88161: f64, t88164: f64) -> (f64, f64) {
    let t88368 = 0.62071215503128080361e4_f64 * t41588 * t88031 * t11387;
    let t88382 = -0.85199506172839506175e-1_f64 * t88144 - 0.82156666666666666667e-1_f64 * t88147 + 0.43816888888888888889e0_f64 * t88150 - 0.43816888888888888888e0_f64 * t77663 + 0.97370864197530864196e-1_f64 * t77667 - 0.107628e2_f64 * t88089 + 0.23917333333333333333e1_f64 * t88097 + t41592 + 0.44291358024691358024e0_f64 * t77499 + 0.39862222222222222223e0_f64 * t77505 - 0.15944888888888888889e1_f64 * t77507 + 0.23917333333333333333e1_f64 * t77509 - 0.98587999999999999998e0_f64 * t88161 - 0.82156666666666666668e-1_f64 * t88164;
    (t88368, t88382)
}
