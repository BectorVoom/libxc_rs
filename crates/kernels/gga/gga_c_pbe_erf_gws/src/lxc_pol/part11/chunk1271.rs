//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1271/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1271<F: Float>(t46536: F, t48985: F, t858: F, t884: F, t886: F, t11414: F, t37965: F, t13252: F, t39052: F, t46549: F, t46566: F, t11540: F) -> (F, F, F, F, F, F, F) {
    let t50275 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t46536;
    let t50279 = t884 * t886 * t858 * t48985 / F::cast_from(48.0_f64);
    let t50281 = t37965 * t11414 / F::cast_from(4.0_f64);
    let t50286 = t39052 * t13252;
    let t50290 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t46549;
    let t50291 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t46566;
    let t50292 = t11540 * t13252;
    (t50275, t50279, t50281, t50286, t50290, t50291, t50292)
}
