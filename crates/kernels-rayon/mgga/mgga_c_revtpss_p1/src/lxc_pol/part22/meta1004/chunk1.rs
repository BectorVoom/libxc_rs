//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3429/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429(t291: f64, t64358: f64, t64372: f64, t64386: f64, t64400: f64, t41908: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64) -> (f64, f64) {
    let t64404 = 0.621814e-1_f64 * (t64358 + t64372 + t64386 + t64400) * t291;
    let t64416 = 0.68493333333333333332e-1_f64 * t63274 - 0.2283111111111111111e-1_f64 * t63276 + 0.76103703703703703701e-2_f64 * t63278 - 0.2283111111111111111e-1_f64 * t63281 - 0.11415555555555555555e-1_f64 * t63285 - 0.19025925925925925925e-1_f64 * t63290 + 0.68493333333333333332e-1_f64 * t63293 + 0.34246666666666666666e-1_f64 * t63299 + 0.2283111111111111111e0_f64 * t63304 - 0.41095999999999999999e0_f64 * t63308 + t41908 + 0.11415555555555555555e-1_f64 * t51967;
    (t64404, t64416)
}
