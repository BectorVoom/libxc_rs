//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3057/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057(t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64) -> f64 {
    let t63424 = 0.2283111111111111111e0_f64 * t63380 + 0.1522074074074074074e-1_f64 * t63382 + 0.4566222222222222222e-1_f64 * t63384 - 0.68493333333333333331e-1_f64 * t63388 - 0.41095999999999999999e0_f64 * t63392 - 0.2283111111111111111e-1_f64 * t63396 - 0.45662222222222222221e-1_f64 * t63398 - 0.68493333333333333332e-1_f64 * t63400 + 0.10274e0_f64 * t63404 + 0.41096e0_f64 * t63408 + 0.68493333333333333332e-1_f64 * t63412 + 0.19025925925925925925e-1_f64 * t63417 - 0.50735802469135802467e-1_f64 * t63422;
    t63424
}
