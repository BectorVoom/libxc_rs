//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1041/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1041(t2629: f64, t3904: f64, t11004: f64, t10982: f64, t10980: f64, t10986: f64, t11002: f64, t11010: f64, t11015: f64, t11020: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8927: f64) -> (f64, f64) {
    let t11269 = 0.11696447245269292414e1_f64 * t2629 * t3904;
    let t11276 = 0.2283111111111111111e-1_f64 * t11004;
    let t11277 = 0.11415555555555555555e-1_f64 * t10982;
    let t11286 = -t8927 - 0.1522074074074074074e-1_f64 * t8616 + 0.38051851851851851851e-2_f64 * t8607 - 0.11415555555555555555e-1_f64 * t8618 + 0.57077777777777777777e-2_f64 * t8605 - 0.76103703703703703702e-2_f64 * t10980 + 0.76103703703703703701e-2_f64 * t11002 - t11276 + t11277 - 0.19025925925925925925e-1_f64 * t11010 + 0.68493333333333333331e-1_f64 * t11015 - 0.2283111111111111111e-1_f64 * t11020 - 0.11415555555555555555e-1_f64 * t11024 - 0.10274e0_f64 * t11028 + 0.68493333333333333332e-1_f64 * t11033 + 0.34246666666666666666e-1_f64 * t11037 - 0.17123333333333333333e-1_f64 * t10986;
    (t11269, t11286)
}
