//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1160/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1160(t139241: f64, t139254: f64, t139257: f64, t139275: f64, t139278: f64, t148334: f64, t148338: f64, t148342: f64, t148346: f64, t148349: f64, t148353: f64, t148360: f64, t148365: f64, t148369: f64, t148373: f64, t148375: f64) -> f64 {
    let t148765 = -4.0_f64 / 9.0_f64 * t148334 - t148338 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t139241 - t148342 / 3.0_f64 - t148346 / 12.0_f64 + t148349 / 6.0_f64 - t148353 + 4.0_f64 / 3.0_f64 * t139254 - t139257 - t139275 / 12.0_f64 + t139278 / 6.0_f64 + 4.0_f64 / 3.0_f64 * t148360 + t148365 / 4.0_f64 + 12.0_f64 * t148369 - 6.0_f64 * t148373 - t148375 / 18.0_f64;
    t148765
}
