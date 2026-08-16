//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 896/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk896(t1730: f64, t2753: f64, t1: f64, t837: f64, t2736: f64, t616: f64, t5459: f64, t5465: f64, t2608: f64, t5493: f64, t1620: f64, t1724: f64, t2607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7775 = 16.0_f64 / 45.0_f64 * t1730 * t2753;
    let t7776 = t1 * t837;
    let t7777 = t7776 * t2736;
    let t7778 = t616 * t7777;
    let t7779 = 4.0_f64 / 9.0_f64 * t7778;
    let t7780 = 16.0_f64 / 135.0_f64 * t5459;
    let t7781 = 16.0_f64 / 405.0_f64 * t5465;
    let t7782 = t5493 * t2608;
    let t7784 = 16.0_f64 / 45.0_f64 * t1620 * t7782;
    let t7785 = t2607 * t1724;
    (t7775, t7776, t7779, t7780, t7781, t7784, t7785)
}
