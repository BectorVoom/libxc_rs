//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 519/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk519<F: Float>(t2775: F, t2795: F, t199: F, t775: F, t13: F, t30: F, t778: F, t2666: F, t27: F, t779: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2796 = t2775 * t2795;
    let t2800 = F::cast_from(1.0_f64) / t775 / t199;
    let t2801 = t13 * t2800;
    let t2803 = F::cast_from(1.0_f64) / t778 / t30;
    let t2804 = t2666 * t2803;
    let t2805 = t2801 * t2804;
    let t2806 = F::cast_from(0.51726012919273400301e3_f64) * t2805;
    let t2808 = F::cast_from(1.0_f64) / t775 / t27;
    let t2809 = t13 * t2808;
    let t2810 = t2666 * t779;
    let t2811 = t2809 * t2810;
    let t2812 = F::cast_from(0.96491876992155210402e2_f64) * t2811;
    (t2796, t2800, t2801, t2803, t2804, t2805, t2806, t2808, t2809, t2810, t2811, t2812)
}
