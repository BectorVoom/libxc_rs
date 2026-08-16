//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1139/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1139(t152826: f64, t193: f64, t2781: f64, t6308: f64, t143329: f64, t143333: f64, t143336: f64, t143339: f64, t143355: f64, t143366: f64, t143371: f64, t153435: f64, t153439: f64, t153443: f64, t153449: f64, t153453: f64, t153456: f64, t153460: f64, t153464: f64) -> (f64, f64) {
    let t153468 = t6308 * t193 * t2781 * t152826;
    let t153470 = -2.0_f64 * t153435 - t153439 / 6.0_f64 - t153443 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t143329 + t143333 - t143336 + t143339 / 9.0_f64 - t143355 / 36.0_f64 + t153449 / 3.0_f64 - t143366 - t143371 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t153453 - 2.0_f64 / 9.0_f64 * t153456 + 4.0_f64 / 3.0_f64 * t153460 - 2.0_f64 / 3.0_f64 * t153464 - t153468 / 6.0_f64;
    (t153468, t153470)
}
