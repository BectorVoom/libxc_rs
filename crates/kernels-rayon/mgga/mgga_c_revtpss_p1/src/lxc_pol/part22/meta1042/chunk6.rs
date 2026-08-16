//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3642/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3642(t58207: f64, t68454: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64, t68548: f64, t68550: f64, t68553: f64, t68556: f64, t68559: f64, t68561: f64) -> f64 {
    let t68920 = 0.43816888888888888889e0_f64 * t68529 - 0.85199506172839506175e-1_f64 * t68532 + 0.32862666666666666666e0_f64 * t68535 - 0.48685432098765432097e-1_f64 * t58207 - 0.43816888888888888888e0_f64 * t68538 - 0.65725333333333333332e0_f64 * t68540 + 0.16431333333333333333e0_f64 * t68543 + 0.49293999999999999999e0_f64 * t68546 + 0.73028148148148148149e-1_f64 * t68548 + 0.21908444444444444444e0_f64 * t68550 - 0.54771111111111111112e-1_f64 * t68553 + 0.36514074074074074075e-1_f64 * t68556 + 0.5696775e1_f64 * t68559 - 0.3071625e0_f64 * t68561 - 0.79724444444444444445e0_f64 * t68454;
    t68920
}
