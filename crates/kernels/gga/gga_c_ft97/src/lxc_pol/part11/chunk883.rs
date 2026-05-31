//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 883/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk883<F: Float>(t1644: F, t1771: F, t458: F, t7956: F, t7974: F, t37315: F, t378: F, t92: F, t1570: F, t37362: F, t37264: F, t38077: F, t38081: F, t38084: F, t38088: F, t38090: F, t38092: F) -> (F, F, F, F, F, F, F, F) {
    let t38094 = t1771 * t1644;
    let t38096 = t458 * t7956;
    let t38098 = t458 * t7974;
    let t38101 = t92 * t378 * t37315;
    let t38103 = t1570 * t37362;
    let t38105 = t92 * t378 * t38103;
    let t38108 = t92 * t378 * t37264;
    let t38110 = F::cast_from(8.0_f64) * t38077 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t38081 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38084 + F::cast_from(8.0_f64) * t38088 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t38090 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38092 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t38094 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t38096 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t38098 - F::cast_from(12.0_f64) * t38101 + F::cast_from(2.0_f64) * t38105 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38108;
    (t38094, t38096, t38098, t38101, t38103, t38105, t38108, t38110)
}
