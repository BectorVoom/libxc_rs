//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1054/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1054(t3221: f64, t9550: f64, t3220: f64, t1123: f64, t6396: f64, t2255: f64, t2253: f64, t2277: f64, t2312: f64, t6477: f64, t9033: f64, t9037: f64, t9039: f64, t9041: f64, t9042: f64, t9539: f64, t9540: f64, t9545: f64, t9549: f64) -> (f64, f64, f64, f64, f64) {
    let t9551 = t3221 * t9550;
    let t9552 = t3220 * t9551;
    let t9555 = t1123 * t6396;
    let t9556 = t2255 * t9555;
    let t9559 = -7.0_f64 / 1152.0_f64 * t6477 + t9539 - t9033 - t2312 * t9540 / 192.0_f64 + t2277 * t9545 / 768.0_f64 + t9549 - t2253 * t9552 / 768.0_f64 - t2253 * t9556 / 768.0_f64 + t9037 - t9039 - t9041 - t9042;
    (t9551, t9552, t9555, t9556, t9559)
}
