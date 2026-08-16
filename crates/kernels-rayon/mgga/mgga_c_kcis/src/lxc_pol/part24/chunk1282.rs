//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1282/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1282(t100108: f64, t100133: f64, t100136: f64, t26692: f64, t28988: f64, t8038: f64, t95572: f64, t95581: f64, t95586: f64, t95587: f64, t95605: f64, t95608: f64, t96227: f64) -> f64 {
    let t100999 = t95572 - 0.58958024691358024688e-2_f64 * t95581 - 0.33163888888888888888e-2_f64 * t100108 + t95586 - 0.22109259259259259259e-2_f64 * t95587 + 0.88437037037037037035e-2_f64 * t100133 + 0.37069444444444444445e-2_f64 * t26692 * t28988 + 0.12356481481481481482e-2_f64 * t96227 * t8038 + 0.27636574074074074073e-2_f64 * t100136 + t95605 + t95608;
    t100999
}
