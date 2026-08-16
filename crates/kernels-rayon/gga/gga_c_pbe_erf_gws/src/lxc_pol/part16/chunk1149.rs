//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1149/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1149(t2494: f64, t4066: f64, t4233: f64, t945: f64, t810: f64, t4209: f64, t4414: f64, t1115: f64, t14198: f64, t14311: f64, t14416: f64, t14426: f64, t14444: f64, t14457: f64, t14464: f64, t14467: f64, t14470: f64, t2498: f64, t3040: f64, t4083: f64) -> (f64, f64, f64, f64) {
    let t14849 = t4066 * t2494;
    let t14852 = t4233 * t945;
    let t14854 = t14852 * t810;
    let t14867 = t4414 * t4209;
    let t14873 = -t2498 * t4083 / 96.0_f64 - t1115 * t14311 / 96.0_f64 - t14416 / 768.0_f64 - t14426 / 768.0_f64 - t3040 * t4083 / 96.0_f64 + 7.0_f64 / 288.0_f64 * t14198 + t14444 / 1536.0_f64 - 7.0_f64 / 144.0_f64 * t14867 + t14457 / 384.0_f64 - t14464 / 24.0_f64 - t14467 / 24.0_f64 - t14470 / 24.0_f64;
    (t14849, t14852, t14854, t14873)
}
