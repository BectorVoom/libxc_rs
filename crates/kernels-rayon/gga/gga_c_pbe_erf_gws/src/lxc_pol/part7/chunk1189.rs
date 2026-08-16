//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1189/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1189(t21121: f64, t854: f64, t19553: f64, t858: f64, t884: f64, t886: f64, t4408: f64, t6638: f64, t20597: f64, t2118: f64, t20085: f64, t21064: f64, t21068: f64, t21106: f64, t21115: f64, t21118: f64, t2272: f64, t2305: f64, t2312: f64, t6207: f64, t6276: f64, t6637: f64, t824: f64, t902: f64, t905: f64) -> (f64, f64, f64) {
    let t21122 = t854 * t21121;
    let t21123 = 455.0_f64 / 324.0_f64 * t21122;
    let t21127 = t884 * t886 * t858 * t19553 / 48.0_f64;
    let t21128 = t4408 * t6638;
    let t21132 = t2118 * t20597;
    let t21139 = -t21064 + t21068 + t902 * t905 * t2305 * t20085 / 256.0_f64 + t902 * t905 * t21106 * t824 / 1536.0_f64 + t21115 - 595.0_f64 / 1296.0_f64 * t21118 - t21123 - t21127 + t6637 * t6276 * t21128 / 96.0_f64 + t6637 * t6276 * t21132 / 128.0_f64 - t2312 * t6207 * t2272 / 64.0_f64;
    (t21123, t21127, t21139)
}
