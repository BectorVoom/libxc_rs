//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1191/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1191(t1452: f64, t814: f64, t2306: f64, t3074: f64, t339: f64, t6104: f64, t860: f64, t6373: f64, t6484: f64, t19993: f64, t20264: f64, t20527: f64, t21146: f64, t21148: f64, t21155: f64, t21158: f64, t21159: f64, t2255: f64, t2277: f64, t2278: f64, t2300: f64, t2343: f64, t3235: f64, t6350: f64, t6598: f64, t6637: f64, t875: f64, t904: f64, t929: f64) -> (f64, f64, f64) {
    let t21161 = t1452 * t814;
    let t21174 = t3074 * t2306 * t6104 * t339 * t860 / 24.0_f64;
    let t21175 = t6484 * t6373;
    let t21176 = 7.0_f64 / 24.0_f64 * t21175;
    let t21181 = -t2277 * t2255 * t6350 * t6598 / 256.0_f64 - 35.0_f64 / 96.0_f64 * t21146 + t6637 * t20527 * t21148 / 32.0_f64 + t21155 - t21158 - 7.0_f64 / 64.0_f64 * t21159 - t2277 * t2255 * t2278 * t21161 / 512.0_f64 + 5.0_f64 / 192.0_f64 * t929 * t2300 * t904 * t19993 + t21174 + t21176 - t2343 * t3235 * t20264 * t875 / 384.0_f64;
    (t21174, t21176, t21181)
}
