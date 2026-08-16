//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1038/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1038(t11598: f64, t11602: f64, t11604: f64, t11606: f64, t11613: f64, t11615: f64, t11620: f64, t11625: f64, t11632: f64, t11635: f64, t2277: f64, t2312: f64, t2343: f64, t8901: f64, t9415: f64, t9425: f64) -> f64 {
    let t11638 = 7.0_f64 / 288.0_f64 * t11598 - t11602 - t9415 - t8901 - t11604 - t2312 * t11606 / 384.0_f64 + t11613 + t2343 * t11615 / 192.0_f64 + t2343 * t11620 / 384.0_f64 + t2277 * t11625 / 768.0_f64 - t11632 - t9425 * t11635 / 128.0_f64;
    t11638
}
