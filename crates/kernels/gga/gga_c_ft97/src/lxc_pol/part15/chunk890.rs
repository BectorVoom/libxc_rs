//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 890/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk890<F: Float>(t295: F, t41752: F, t10478: F, t871: F, t2770: F, t2843: F, t10491: F, t870: F, t9577: F, t2347: F, t2842: F, t10695: F, t311: F) -> (F, F, F, F, F, F, F) {
    let t44445 = t41752 * t295;
    let t44518 = t10478 * t871;
    let t44523 = t2770 * t2843;
    let t44528 = t10491 * t871;
    let t44533 = t870 * t9577;
    let t44566 = t2842 * t2347;
    let t44600 = F::cast_from(1.0_f64) / t10695 / t311;
    (t44445, t44518, t44523, t44528, t44533, t44566, t44600)
}
