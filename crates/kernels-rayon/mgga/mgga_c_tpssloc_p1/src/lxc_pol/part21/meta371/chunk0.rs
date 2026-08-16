//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1817/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1817(t4392: f64, t699: f64, t13611: f64, t908: f64, t136: f64, t13602: f64, t13598: f64, t13613: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13644 = t699 * t4392;
    let t13645 = 0.10954222222222222222e0_f64 * t13644;
    let t13646 = t908 * t13611;
    let t13647 = t136 * t13646;
    let t13650 = 0.19931111111111111111e0_f64 * t13602;
    let t13652 = -0.1898925e1_f64 * t13630 - 0.9494625e0_f64 * t13632 + 0.142419375e1_f64 * t13635 - 0.76790625e-1_f64 * t13638 + 0.1898925e1_f64 * t13640 - 0.91285185185185185185e-1_f64 * t13642 + t13645 - 0.82156666666666666667e-1_f64 * t13647 - 0.13287407407407407408e0_f64 * t13598 + t13650 - 0.29896666666666666667e0_f64 * t13613;
    (t13644, t13645, t13646, t13647, t13650, t13652)
}
