//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 224/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk224(t10: f64, t671: f64, t670: f64, t395: f64, t401: f64, t7: f64, t226: f64, t230: f64, t242: f64, t528: f64, t3: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t672 = t10 * t671;
    let t674 = 0.10821041362364843377e0_f64 * t670 * t672;
    let t677 = 0.4125e0_f64 * t395 - t401 / 6.0_f64;
    let t678 = t677 * pi;
    let t679 = t678 * t7;
    let t681 = 4.0_f64 / 3.0_f64 * t226 * t679;
    let t683 = 4.0_f64 / 3.0_f64 * t226 * t230;
    let t692 = 0.83762820535504401876e-1_f64 * t528 * t242;
    let t696 = t551 * t3;
    (t672, t674, t677, t678, t679, t681, t683, t692, t696)
}
