//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1069/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1069(t10: f64, t16423: f64, t506: f64, t119: f64, t1504: f64, t331: f64, t5803: f64, t5813: f64, t155: f64, t5645: f64, t1513: f64, t1533: f64, t299: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19274 = t10 * t506 * t16423;
    let t19278 = t119 * t331 * t1504;
    let t19279 = t5803 * t19278;
    let t19281 = t5813 * t19278;
    let t19282 = 0.77947333333333333333e1_f64 * t19281;
    let t19284 = t119 * t155 * t5645;
    let t19285 = t1513 * t19284;
    let t19286 = 0.19486833333333333333e1_f64 * t19285;
    let t19288 = t799 * t299 * t1533;
    (t19274, t19279, t19282, t19284, t19286, t19288)
}
