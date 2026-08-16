//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 710/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk710(t28501: f64, t2862: f64, t319: f64, t28506: f64, t1882: f64, t7116: f64, t7107: f64, t29285: f64, t29287: f64, t29290: f64, t29295: f64, t29299: f64, t29304: f64, t29309: f64, t29313: f64, t29317: f64, t29321: f64, t446: f64) -> f64 {
    let t29325 = t2862 * t319 * t28501;
    let t29329 = t2862 * t319 * t28506;
    let t29332 = t1882 * t7116;
    let t29334 = t1882 * t7107;
    let t29336 = t29285 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t29287 + 2.0_f64 / 3.0_f64 * t446 * t29290 + t446 * t29295 / 3.0_f64 + t446 * t29299 / 3.0_f64 + t446 * t29304 / 3.0_f64 + t446 * t29309 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29313 + t446 * t29317 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t29321 + 2.0_f64 / 3.0_f64 * t446 * t29325 + 2.0_f64 / 3.0_f64 * t446 * t29329 - 2.0_f64 / 9.0_f64 * t29332 - t29334 / 9.0_f64;
    t29336
}
