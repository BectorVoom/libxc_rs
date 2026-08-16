//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3057/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057<F: Float>(t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F) -> F {
    let t63424 = F::cast_from(0.2283111111111111111e0_f64) * t63380 + F::cast_from(0.1522074074074074074e-1_f64) * t63382 + F::cast_from(0.4566222222222222222e-1_f64) * t63384 - F::cast_from(0.68493333333333333331e-1_f64) * t63388 - F::cast_from(0.41095999999999999999e0_f64) * t63392 - F::cast_from(0.2283111111111111111e-1_f64) * t63396 - F::cast_from(0.45662222222222222221e-1_f64) * t63398 - F::cast_from(0.68493333333333333332e-1_f64) * t63400 + F::cast_from(0.10274e0_f64) * t63404 + F::cast_from(0.41096e0_f64) * t63408 + F::cast_from(0.68493333333333333332e-1_f64) * t63412 + F::cast_from(0.19025925925925925925e-1_f64) * t63417 - F::cast_from(0.50735802469135802467e-1_f64) * t63422;
    t63424
}
