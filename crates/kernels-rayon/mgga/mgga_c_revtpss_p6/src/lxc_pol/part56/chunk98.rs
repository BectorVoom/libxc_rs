//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 98/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk98(t281: f64, t282: f64, t283: f64, t273: f64, t276: f64, t279: f64, t275: f64, t153: f64, t159: f64, t162: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t285 = t281 * t282 * t283;
    let t287 = 0.379785e1_f64 * t276 + 0.8969e0_f64 * t273 + 0.204775e0_f64 * t279 + 0.123235e0_f64 * t285;
    let t290 = 1.0_f64 + 0.16081979498692535067e2_f64 / t287;
    let t291 = f64::ln(t290);
    let t293 = 0.621814e-1_f64 * t275 * t291;
    let t294 = 2.0_f64 <= zeta_threshold;
    let t296 = piecewise3(t294, t153, 2.0_f64 * t159);
    let t297 = 0.0_f64 <= zeta_threshold;
    let t298 = piecewise3(t297, t153, 0.0_f64);
    let t300 = (t296 + t298 - 2.0_f64) * t162;
    (t285, t287, t290, t291, t293, t300)
}
