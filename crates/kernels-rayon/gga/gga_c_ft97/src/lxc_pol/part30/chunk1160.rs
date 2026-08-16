//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1160/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1160(t143276: f64, t143321: f64, t143324: f64, t143327: f64, t143500: f64, t153388: f64, t153390: f64, t153395: f64, t153399: f64, t153402: f64, t153405: f64, t153414: f64, t153418: f64, t153422: f64, t153427: f64, t153431: f64) -> f64 {
    let t154204 = -t143500 - t153388 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t153390 + t153395 / 6.0_f64 - t153399 + t153402 / 3.0_f64 - t153405 / 3.0_f64 - t143276 + 2.0_f64 * t143321 - 4.0_f64 / 3.0_f64 * t143324 - 2.0_f64 / 3.0_f64 * t143327 - 20.0_f64 * t153414 + 8.0_f64 * t153418 - 6.0_f64 * t153422 - 4.0_f64 / 3.0_f64 * t153427 + 3.0_f64 * t153431;
    t154204
}
