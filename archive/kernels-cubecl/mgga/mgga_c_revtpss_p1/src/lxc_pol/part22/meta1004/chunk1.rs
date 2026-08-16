//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3429/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3429<F: Float>(t291: F, t64358: F, t64372: F, t64386: F, t64400: F, t41908: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> (F, F) {
    let t64404 = F::cast_from(0.621814e-1_f64) * (t64358 + t64372 + t64386 + t64400) * t291;
    let t64416 = F::cast_from(0.68493333333333333332e-1_f64) * t63274 - F::cast_from(0.2283111111111111111e-1_f64) * t63276 + F::cast_from(0.76103703703703703701e-2_f64) * t63278 - F::cast_from(0.2283111111111111111e-1_f64) * t63281 - F::cast_from(0.11415555555555555555e-1_f64) * t63285 - F::cast_from(0.19025925925925925925e-1_f64) * t63290 + F::cast_from(0.68493333333333333332e-1_f64) * t63293 + F::cast_from(0.34246666666666666666e-1_f64) * t63299 + F::cast_from(0.2283111111111111111e0_f64) * t63304 - F::cast_from(0.41095999999999999999e0_f64) * t63308 + t41908 + F::cast_from(0.11415555555555555555e-1_f64) * t51967;
    (t64404, t64416)
}
