//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1051/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1051(t3252: f64, t9521: f64, t1123: f64, t2255: f64, t6211: f64, t2312: f64, t6275: f64, t6403: f64, t6579: f64, t6637: f64, t8980: f64, t8985: f64, t8993: f64, t8998: f64, t9002: f64, t9007: f64, t9506: f64, t9509: f64, t9512: f64, t9517: f64) -> (f64, f64, f64) {
    let t9522 = t3252 * t9521;
    let t9527 = t2255 * t1123 * t6211;
    let t9530 = t8980 + t8985 + t8993 + t6275 * t9506 / 96.0_f64 + t6637 * t9509 / 384.0_f64 - t8998 + 5.0_f64 / 192.0_f64 * t6579 * t9512 - t9002 + t9007 - t2312 * t9517 / 96.0_f64 + t2312 * t9522 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t6403 - t2312 * t9527 / 192.0_f64;
    (t9522, t9527, t9530)
}
