//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1187/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1187(t3717: f64, t4066: f64, t1105: f64, t4227: f64, t2376: f64, t2409: f64, t1205: f64, t3721: f64, t9296: f64, t14605: f64, t14867: f64, t14888: f64, t14914: f64, t15036: f64, t15135: f64, t15147: f64, t15152: f64, t15162: f64, t15165: f64, t15170: f64, t15178: f64, t15183: f64, t15187: f64, t2408: f64, t3066: f64, t3917: f64, t4083: f64, t8793: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15406 = t4066 * t3717;
    let t15423 = t4227 * t1105;
    let t15425 = t2409 * t2376 * t15423;
    let t15429 = t1205 * t3721;
    let t15431 = t2409 * t9296 * t15429;
    let t15437 = -t15135 / 384.0_f64 - t15147 / 384.0_f64 - t15152 / 768.0_f64 + t8793 * t14888 / 24.0_f64 + t8793 * t15036 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t14867 + t15162 / 48.0_f64 + t15165 / 24.0_f64 + t15170 / 768.0_f64 - t15178 / 1536.0_f64 + t15183 / 192.0_f64 + 7.0_f64 / 1152.0_f64 * t14605 + t2408 * t15425 / 24.0_f64 + t15187 / 768.0_f64 - t3066 * t15431 / 16.0_f64 - t3917 * t4083 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t14914;
    (t15406, t15423, t15425, t15429, t15431, t15437)
}
