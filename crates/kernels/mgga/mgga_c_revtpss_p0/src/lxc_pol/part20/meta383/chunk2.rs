//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1399/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399<F: Float>(t10506: F, t41020: F, t10495: F, t11008: F, t2765: F, t2771: F, t2828: F, t40978: F, t40982: F, t40986: F, t40988: F, t40994: F, t40998: F, t40999: F, t41003: F, t41004: F, t41006: F, t41008: F, t41014: F, t41018: F, t865: F, t887: F) -> F {
    let t41021 = t41020 * t10506;
    let t41023 = -F::cast_from(0.78548797528808629095e-3_f64) * t40978 - F::cast_from(0.21951497276451705328e-1_f64) * t40982 + F::cast_from(0.15805078039045227836e2_f64) * t2765 * t10495 - F::cast_from(0.43902994552903410657e-1_f64) * t40986 - F::cast_from(0.68293547082294194357e-1_f64) * t40988 - F::cast_from(0.23707617058567841754e2_f64) * t865 * t11008 * t2771 * t2828 + F::cast_from(0.43902994552903410657e-1_f64) * t40994 - t40998 - F::cast_from(0.87805989105806821314e-1_f64) * t40999 - t41003 + F::cast_from(0.68293547082294194357e-1_f64) * t41004 + F::cast_from(0.87805989105806821314e-1_f64) * t41006 - F::cast_from(0.26341796731742046395e1_f64) * t41008 * t887 + F::cast_from(0.13878983423218070567e-1_f64) * t41014 - F::cast_from(0.11708928647259339623e0_f64) * t41018 - F::cast_from(0.13878983423218070567e-1_f64) * t41021;
    t41023
}
