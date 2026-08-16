//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1308/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1308(t106080: f64, t106082: f64, t106090: f64, t106102: f64, t113222: f64, t113226: f64, t113228: f64, t113230: f64, t113232: f64, t113235: f64, t113237: f64, t93021: f64, t99091: f64, t99113: f64) -> f64 {
    let t113240 = -t93021 - 0.76230004213927992339e-4_f64 * t106080 - 7.0_f64 / 16.0_f64 * t106082 - t113222 / 4.0_f64 + 7.0_f64 / 48.0_f64 * t106090 - 0.18292914397043087774e-2_f64 * t99091 + 0.25724410870841842184e-1_f64 * t113226 + 0.25724410870841842183e-2_f64 * t113228 - 0.42874018118069736972e-3_f64 * t113230 - 0.51448821741683684367e-1_f64 * t113232 - 0.27107389498472794076e-4_f64 * t99113 - 0.42874018118069736972e-3_f64 * t113235 - 0.25724410870841842183e-2_f64 * t113237 - 0.85748036236139473943e-3_f64 * t106102;
    t113240
}
