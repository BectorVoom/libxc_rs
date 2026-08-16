//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1214/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1214(t102981: f64, t102994: f64, t103001: f64, t103009: f64, t110236: f64, t110245: f64, t110247: f64, t110276: f64, t95542: f64, t95548: f64, t95567: f64, t95569: f64, t95607: f64, t95632: f64) -> f64 {
    let t115493 = 0.86736281882051994623e-1_f64 * t110236 - 0.16463622957338778996e-1_f64 * t110245 - t95542 - 0.15421710918628844643e0_f64 * t110247 - t95548 + t95567 + t95569 - 0.10281140612419229763e-1_f64 * t102981 - 0.23132566377943266966e0_f64 * t110276 + 0.28912093960683998208e-1_f64 * t102994 - t95607 - 0.51405703062096148814e-2_f64 * t103001 + 0.13709901006661042888e-1_f64 * t103009 + t95632;
    t115493
}
