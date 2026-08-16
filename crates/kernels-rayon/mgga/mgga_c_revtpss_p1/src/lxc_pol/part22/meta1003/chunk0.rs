//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3418/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3418(t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t63377: f64) -> f64 {
    let t64244 = 0.123954e2_f64 * t63336 - 0.13772666666666666667e1_f64 * t63338 + 0.45908888888888888889e0_f64 * t63340 + 0.38257407407407407407e0_f64 * t63342 - 0.57386111111111111112e0_f64 * t63346 - 0.15302962962962962963e1_f64 * t63351 + 0.20659e1_f64 * t63355 - 0.68863333333333333334e0_f64 * t63359 + 0.20659e1_f64 * t63361 + 0.20659e1_f64 * t63366 - 0.309885e1_f64 * t63369 - 0.13772666666666666667e1_f64 * t63371 - 0.309885e1_f64 * t63374 - 0.250068e1_f64 * t63377 + 0.20659e1_f64 * t52033;
    t64244
}
