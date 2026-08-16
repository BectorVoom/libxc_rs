//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 830/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk830(t2608: f64, t5493: f64, t1620: f64, t2825: f64, t586: f64, t1006: f64, t1740: f64, t1033: f64, t1778: f64, t7280: f64, t1045: f64, t1672: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7782 = t5493 * t2608;
    let t7784 = 16.0_f64 / 45.0_f64 * t1620 * t7782;
    let t7793 = t2825 * t586;
    let t7810 = 8.0_f64 / 45.0_f64 * t1006 * t1740;
    let t7811 = t1033 * t1778;
    let t7819 = 0.2518888888888888889e-2_f64 * t7280;
    let t7844 = t1672 * t1045;
    (t7784, t7793, t7810, t7811, t7819, t7844)
}
