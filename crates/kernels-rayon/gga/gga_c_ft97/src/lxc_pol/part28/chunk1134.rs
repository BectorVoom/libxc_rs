//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1134/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1134(t32924: f64, t3450: f64, t40830: f64, t5899: f64, t26909: f64, t5900: f64, t9432: f64, t23649: f64, t34850: f64, t139241: f64, t139254: f64, t139257: f64, t139275: f64, t139278: f64, t148334: f64, t148338: f64, t148342: f64, t148346: f64, t148349: f64, t148353: f64, t148360: f64, t148365: f64) -> (f64, f64, f64, f64) {
    let t148369 = t5899 * t40830 * t32924 * t3450;
    let t148373 = t5899 * t9432 * t5900 * t26909;
    let t148375 = t23649 * t34850;
    let t148377 = -4.0_f64 / 27.0_f64 * t148334 - t148338 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t139241 - t148342 / 9.0_f64 - t148346 / 36.0_f64 + t148349 / 18.0_f64 - t148353 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t139254 - t139257 / 3.0_f64 - t139275 / 36.0_f64 + t139278 / 18.0_f64 + 4.0_f64 / 9.0_f64 * t148360 + t148365 / 12.0_f64 + 4.0_f64 * t148369 - 2.0_f64 * t148373 - t148375 / 54.0_f64;
    (t148369, t148373, t148375, t148377)
}
