//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1065/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1065(t11957: f64, t3138: f64, t3166: f64, t3219: f64, t3235: f64, t3703: f64, t6: f64, t6366: f64, t875: f64, t11514: f64, t11944: f64, t11947: f64, t11949: f64, t11953: f64, t2277: f64, t2312: f64, t2343: f64, t6592: f64, t6597: f64, t9592: f64, t9598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11959 = t3138 * t11957 / 24.0_f64;
    let t11961 = t3235 * t3219 * t3166;
    let t11964 = t6 * t3703;
    let t11966 = t6366 * t11964 * t875;
    let t11970 = t3235 * t11514 * t875;
    let t11973 = -35.0_f64 / 1152.0_f64 * t11944 + t11947 - t6592 - t6597 - t2277 * t11949 / 1536.0_f64 - t2312 * t11953 / 192.0_f64 + t9592 + t11959 - t9598 - t2343 * t11961 / 768.0_f64 - 5.0_f64 / 384.0_f64 * t2343 * t11966 - t2343 * t11970 / 1536.0_f64;
    (t11959, t11961, t11964, t11966, t11970, t11973)
}
