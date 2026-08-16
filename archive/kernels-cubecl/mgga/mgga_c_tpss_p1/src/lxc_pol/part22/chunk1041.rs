//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1041/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1041<F: Float>(t2629: F, t3904: F, t11004: F, t10982: F, t10980: F, t10986: F, t11002: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8927: F) -> (F, F) {
    let t11269 = F::cast_from(0.11696447245269292414e1_f64) * t2629 * t3904;
    let t11276 = F::cast_from(0.2283111111111111111e-1_f64) * t11004;
    let t11277 = F::cast_from(0.11415555555555555555e-1_f64) * t10982;
    let t11286 = -t8927 - F::cast_from(0.1522074074074074074e-1_f64) * t8616 + F::cast_from(0.38051851851851851851e-2_f64) * t8607 - F::cast_from(0.11415555555555555555e-1_f64) * t8618 + F::cast_from(0.57077777777777777777e-2_f64) * t8605 - F::cast_from(0.76103703703703703702e-2_f64) * t10980 + F::cast_from(0.76103703703703703701e-2_f64) * t11002 - t11276 + t11277 - F::cast_from(0.19025925925925925925e-1_f64) * t11010 + F::cast_from(0.68493333333333333331e-1_f64) * t11015 - F::cast_from(0.2283111111111111111e-1_f64) * t11020 - F::cast_from(0.11415555555555555555e-1_f64) * t11024 - F::cast_from(0.10274e0_f64) * t11028 + F::cast_from(0.68493333333333333332e-1_f64) * t11033 + F::cast_from(0.34246666666666666666e-1_f64) * t11037 - F::cast_from(0.17123333333333333333e-1_f64) * t10986;
    (t11269, t11286)
}
