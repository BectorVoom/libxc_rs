//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 829/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk829<F: Float>(t2608: F, t5493: F, t1620: F, t2825: F, t586: F, t1006: F, t1740: F, t1033: F, t1778: F, t7280: F, t1045: F, t1672: F) -> (F, F, F, F, F, F) {
    let t7782 = t5493 * t2608;
    let t7784 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t7782;
    let t7793 = t2825 * t586;
    let t7810 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1006 * t1740;
    let t7811 = t1033 * t1778;
    let t7819 = F::cast_from(0.2518888888888888889e-2_f64) * t7280;
    let t7844 = t1672 * t1045;
    (t7784, t7793, t7810, t7811, t7819, t7844)
}
