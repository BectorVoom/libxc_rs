//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 91/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk91(t195: f64, t198: f64, t222: f64, t251: f64, t258: f64, t266: f64, t273: f64, t4: f64, t71: f64, t84: f64) -> f64 {
    let t276 = 0.53237641966666666666e-3_f64 * t4 * t195 * t71 + 1.0_f64 * t251 * t258 - t198 - t222 + 0.18311447306006545054e-3_f64 * t4 * t195 * t84 + 0.5848223622634646207e0_f64 * t266 * t273;
    t276
}
