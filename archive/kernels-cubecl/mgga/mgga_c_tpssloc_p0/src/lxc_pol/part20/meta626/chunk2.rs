//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2258/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2258<F: Float>(t13025: F, t9546: F, t210: F, t214: F, t41190: F, t41192: F, t41194: F, t41197: F, t41200: F, t46426: F, t46764: F, t46769: F, t46770: F, t46772: F, t46780: F, t787: F) -> F {
    let t46782 = t9546 * t13025;
    let t46783 = F::cast_from(0.15833333333333333333e-1_f64) * t46782;
    let t46784 = F::cast_from(0.98611111111111111108e-1_f64) * t41190 - F::cast_from(0.15833333333333333332e-1_f64) * t41192 + F::cast_from(0.11666666666666666666e0_f64) * t41194 + F::cast_from(0.47499999999999999998e-1_f64) * t41197 - F::cast_from(0.19999999999999999999e-1_f64) * t46764 + t46769 - F::cast_from(0.38888888888888888887e-1_f64) * t46770 + F::cast_from(0.32870370370370370369e-1_f64) * t46772 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t210 * t214 * t46426 + F::cast_from(0.24999999999999999999e-2_f64) * t46780 - t46783 - t41200;
    t46784
}
