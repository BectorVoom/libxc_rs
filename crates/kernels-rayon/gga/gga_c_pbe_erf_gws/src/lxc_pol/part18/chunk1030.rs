//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1030/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1030(t11529: f64, t3258: f64, t2255: f64, t2257: f64, t3781: f64, t11492: f64, t11494: f64, t11495: f64, t11497: f64, t11502: f64, t11506: f64, t11513: f64, t11516: f64, t11519: f64, t11521: f64, t11526: f64, t2253: f64, t2277: f64, t2312: f64, t2343: f64, t3247: f64, t6579: f64, t902: f64) -> (f64, f64, f64, f64, f64) {
    let t11530 = t3258 * t11529;
    let t11531 = t2255 * t11530;
    let t11534 = t3781 * t2257;
    let t11535 = t2255 * t11534;
    let t11538 = -t11492 + t11494 + 7.0_f64 / 2304.0_f64 * t11495 + 7.0_f64 / 2304.0_f64 * t11497 + t2277 * t11502 / 384.0_f64 + t902 * t11506 / 1536.0_f64 + t11513 + t2343 * t11516 / 384.0_f64 - t11519 + t3247 * t11521 / 512.0_f64 + 5.0_f64 / 192.0_f64 * t6579 * t11526 + t2312 * t11531 / 192.0_f64 - t2253 * t11535 / 768.0_f64;
    (t11530, t11531, t11534, t11535, t11538)
}
