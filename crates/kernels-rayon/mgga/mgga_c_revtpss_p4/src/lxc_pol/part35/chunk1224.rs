//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1224/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1224(t106080: f64, t106082: f64, t106090: f64, t106102: f64, t113222: f64, t113226: f64, t113228: f64, t113230: f64, t113232: f64, t113235: f64, t113237: f64, t95684: f64, t99091: f64, t99113: f64) -> f64 {
    let t115712 = -t95684 - 0.15246000842785598468e-3_f64 * t106080 - 7.0_f64 / 8.0_f64 * t106082 - t113222 / 2.0_f64 + 7.0_f64 / 24.0_f64 * t106090 - 0.3658582879408617555e-2_f64 * t99091 + 0.51448821741683684367e-1_f64 * t113226 + 0.51448821741683684367e-2_f64 * t113228 - 0.85748036236139473944e-3_f64 * t113230 - 0.10289764348336736873e0_f64 * t113232 - 0.54214778996945588151e-4_f64 * t99113 - 0.85748036236139473944e-3_f64 * t113235 - 0.51448821741683684367e-2_f64 * t113237 - 0.17149607247227894789e-2_f64 * t106102;
    t115712
}
