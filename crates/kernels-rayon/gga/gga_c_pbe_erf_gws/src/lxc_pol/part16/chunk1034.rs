//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1034/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1034(t1161: f64, t2352: f64, t2409: f64, t3067: f64, t1105: f64, t2376: f64, t274: f64, t745: f64, t820: f64, t3258: f64, t3257: f64, t1123: f64, t6686: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9321 = t1161 * t2352;
    let t9323 = t2409 * t3067 * t9321;
    let t9326 = t1105 * t2352;
    let t9328 = t2409 * t2376 * t9326;
    let t9332 = t745 * t820 * t274;
    let t9333 = t3258 * t9332;
    let t9334 = t3257 * t9333;
    let t9337 = t1123 * t6686;
    (t9321, t9323, t9326, t9328, t9333, t9334, t9337)
}
