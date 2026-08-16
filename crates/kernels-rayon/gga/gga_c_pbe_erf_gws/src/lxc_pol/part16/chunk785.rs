//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 785/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk785(t5803: f64, t5805: f64, t119: f64, t331: f64, t481: f64, t1557: f64, t1: f64, t128: f64, t485: f64, t1513: f64, t1544: f64, t156: f64) -> (f64, f64, f64, f64, f64) {
    let t5806 = t5803 * t5805;
    let t5809 = t119 * t331 * t481;
    let t5810 = t1557 * t5809;
    let t5813 = t485 * t128 * t1;
    let t5814 = t5813 * t5805;
    let t5816 = t1513 * t5809;
    let t5818 = t156 * t1544;
    (t5806, t5810, t5814, t5816, t5818)
}
