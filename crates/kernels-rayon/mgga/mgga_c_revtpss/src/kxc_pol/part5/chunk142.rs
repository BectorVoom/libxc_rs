//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 142/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk142(t281: f64, t282: f64, t414: f64, t406: f64, t409: f64, t412: f64) -> (f64, f64, f64, f64) {
    let t416 = t281 * t282 * t414;
    let t418 = 0.379785e1_f64 * t409 + 0.8969e0_f64 * t406 + 0.204775e0_f64 * t412 + 0.123235e0_f64 * t416;
    let t421 = 1.0_f64 + 0.16081979498692535067e2_f64 / t418;
    let t422 = f64::ln(t421);
    (t416, t418, t421, t422)
}
