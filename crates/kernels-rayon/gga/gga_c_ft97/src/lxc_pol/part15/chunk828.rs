//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 828/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk828(t22161: f64, t799: f64, t27: f64, t89: f64, t19278: f64, t21960: f64, t21964: f64, t21967: f64, t21971: f64, t21975: f64, t21981: f64, t21984: f64, t21987: f64, t21991: f64, t21994: f64) -> (f64, f64, f64) {
    let t22162 = t799 * t22161;
    let t22164 = t89 * t27 * t22162;
    let t22166 = t21960 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t21964 - t21967 / 9.0_f64 + t21971 / 6.0_f64 + t21975 / 6.0_f64 - t19278 / 9.0_f64 - t21981 + t21984 - t21987 / 18.0_f64 - t21991 / 3.0_f64 + t21994 / 3.0_f64 - t22164 / 6.0_f64;
    (t22162, t22164, t22166)
}
