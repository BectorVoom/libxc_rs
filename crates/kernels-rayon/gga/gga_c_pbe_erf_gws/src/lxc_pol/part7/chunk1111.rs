//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1111/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1111(t2306: f64, t4383: f64, t2382: f64, t4395: f64, t19875: f64, t19878: f64, t19880: f64, t19888: f64, t19890: f64, t19892: f64, t2373: f64, t2408: f64, t2409: f64, t4390: f64, t4397: f64, t4459: f64, t4464: f64, t4484: f64, t6112: f64, t6138: f64, t6797: f64, t8734: f64) -> (f64, f64) {
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    let t19898 = t4395 * t4383;
    let t19899 = t2382 * t19898;
    let t19904 = -t2408 * t2409 * t8734 * t6138 / 2.0_f64 + 35.0_f64 / 36.0_f64 * t19875 + 35.0_f64 / 72.0_f64 * t19878 - 35.0_f64 / 36.0_f64 * t19880 - t6112 * t2373 / 12.0_f64 - t4397 * t4459 / 8.0_f64 - t4397 * t4464 / 24.0_f64 - 7.0_f64 / 12.0_f64 * t19888 + 7.0_f64 / 12.0_f64 * t19890 - 7.0_f64 / 12.0_f64 * t19892 + t19895 * t6797 / 4.0_f64 + t19899 * t4390 / 4.0_f64 + t19899 * t4484 / 8.0_f64;
    (t19894, t19904)
}
