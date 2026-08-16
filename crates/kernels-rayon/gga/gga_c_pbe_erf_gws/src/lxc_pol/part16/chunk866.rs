//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 866/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk866(t7373: f64, t7411: f64, t598: f64, t186: f64, t185: f64, t5278: f64, t5281: f64, t5285: f64, t5315: f64, t1006: f64, t1673: f64, t5317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7412 = t7373 + t7411;
    let t7413 = t598 * t7412;
    let t7414 = t186 * t7413;
    let t7416 = 2.0_f64 / 15.0_f64 * t185 * t7414;
    let t7417 = 8.0_f64 / 45.0_f64 * t5278;
    let t7418 = 4.0_f64 / 45.0_f64 * t5281;
    let t7419 = 8.0_f64 / 81.0_f64 * t5285;
    let t7420 = 16.0_f64 / 135.0_f64 * t5315;
    let t7421 = t1006 * t1673;
    let t7422 = 4.0_f64 / 135.0_f64 * t7421;
    let t7423 = 8.0_f64 / 45.0_f64 * t5317;
    (t7416, t7417, t7418, t7419, t7420, t7422, t7423)
}
