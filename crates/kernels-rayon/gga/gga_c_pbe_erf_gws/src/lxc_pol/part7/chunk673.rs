//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 673/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk673(t1903: f64, t720: f64, t254: f64, t542: f64, t252: f64, t1907: f64, t723: f64, t245: f64, t713: f64, t1802: f64, t610: f64, t1866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5384 = 2.0_f64 / 9.0_f64 * t720 * t1903;
    let t5385 = t254 * t542;
    let t5387 = 8.0_f64 / 81.0_f64 * t252 * t5385;
    let t5388 = t1907 * t723;
    let t5390 = t245 * t713;
    let t5393 = t1802 * t610;
    let t5394 = t5393 * t1866;
    (t5384, t5385, t5387, t5388, t5390, t5393, t5394)
}
