//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2020/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2020(t103315: f64, t103316: f64, t103318: f64, t103320: f64, t103324: f64, t106080: f64, t106082: f64, t106085: f64, t106088: f64, t106090: f64, t93035: f64, t95684: f64) -> f64 {
    let t110429 = -t95684 - 0.50820002809285328225e-4_f64 * t106080 - 7.0_f64 / 24.0_f64 * t106082 - t103315 - t103316 + t103318 - t103320 + t103324 + 0.34299214494455789578e-2_f64 * t106085 + 0.54208002996571016773e-3_f64 * t93035 + 0.68598428988911579156e-2_f64 * t106088 + 7.0_f64 / 72.0_f64 * t106090;
    t110429
}
