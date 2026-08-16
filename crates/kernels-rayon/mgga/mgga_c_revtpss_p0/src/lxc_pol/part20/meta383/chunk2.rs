//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1399/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1399(t10506: f64, t41020: f64, t10495: f64, t11008: f64, t2765: f64, t2771: f64, t2828: f64, t40978: f64, t40982: f64, t40986: f64, t40988: f64, t40994: f64, t40998: f64, t40999: f64, t41003: f64, t41004: f64, t41006: f64, t41008: f64, t41014: f64, t41018: f64, t865: f64, t887: f64) -> f64 {
    let t41021 = t41020 * t10506;
    let t41023 = -0.78548797528808629095e-3_f64 * t40978 - 0.21951497276451705328e-1_f64 * t40982 + 0.15805078039045227836e2_f64 * t2765 * t10495 - 0.43902994552903410657e-1_f64 * t40986 - 0.68293547082294194357e-1_f64 * t40988 - 0.23707617058567841754e2_f64 * t865 * t11008 * t2771 * t2828 + 0.43902994552903410657e-1_f64 * t40994 - t40998 - 0.87805989105806821314e-1_f64 * t40999 - t41003 + 0.68293547082294194357e-1_f64 * t41004 + 0.87805989105806821314e-1_f64 * t41006 - 0.26341796731742046395e1_f64 * t41008 * t887 + 0.13878983423218070567e-1_f64 * t41014 - 0.11708928647259339623e0_f64 * t41018 - 0.13878983423218070567e-1_f64 * t41021;
    t41023
}
