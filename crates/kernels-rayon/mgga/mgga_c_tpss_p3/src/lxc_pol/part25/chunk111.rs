//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 111/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk111(t275: f64, t276: f64, t277: f64, t267: f64, t270: f64, t273: f64) -> (f64, f64, f64, f64) {
    let t279 = t275 * t276 * t277;
    let t281 = 0.379785e1_f64 * t270 + 0.8969e0_f64 * t267 + 0.204775e0_f64 * t273 + 0.123235e0_f64 * t279;
    let t284 = 1.0_f64 + 0.16081979498692535067e2_f64 / t281;
    let t285 = f64::ln(t284);
    (t279, t281, t284, t285)
}
