//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2166/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2166(t99009: f64, t99012: f64, t99013: f64, t99015: f64, t99017: f64, t99020: f64, t99022: f64, t99024: f64, t99027: f64, t99030: f64, t99031: f64, t99034: f64, t99035: f64) -> f64 {
    let t99037 = -0.45351183609335988442e-1_f64 * t99009 + t99012 + 0.10841600599314203355e-2_f64 * t99013 + 0.17149607247227894789e-2_f64 * t99015 - 0.85748036236139473944e-3_f64 * t99017 + t99020 - t99022 - t99024 - t99027 + t99030 - 0.51448821741683684367e-1_f64 * t99031 + t99034 - 0.11337795902333997111e-1_f64 * t99035;
    t99037
}
