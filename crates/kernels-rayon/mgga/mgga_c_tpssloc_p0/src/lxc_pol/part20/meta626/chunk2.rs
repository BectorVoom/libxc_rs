//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2258/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2258(t13025: f64, t9546: f64, t210: f64, t214: f64, t41190: f64, t41192: f64, t41194: f64, t41197: f64, t41200: f64, t46426: f64, t46764: f64, t46769: f64, t46770: f64, t46772: f64, t46780: f64, t787: f64) -> f64 {
    let t46782 = t9546 * t13025;
    let t46783 = 0.15833333333333333333e-1_f64 * t46782;
    let t46784 = 0.98611111111111111108e-1_f64 * t41190 - 0.15833333333333333332e-1_f64 * t41192 + 0.11666666666666666666e0_f64 * t41194 + 0.47499999999999999998e-1_f64 * t41197 - 0.19999999999999999999e-1_f64 * t46764 + t46769 - 0.38888888888888888887e-1_f64 * t46770 + 0.32870370370370370369e-1_f64 * t46772 - 0.16666666666666666666e-2_f64 * t787 * t210 * t214 * t46426 + 0.24999999999999999999e-2_f64 * t46780 - t46783 - t41200;
    t46784
}
