//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1063/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1063(t1076: f64, t810: f64, t1123: f64, t2255: f64, t11464: f64, t3235: f64, t875: f64, t11514: f64, t2345: f64, t6287: f64, t11901: f64, t11907: f64, t11911: f64, t11913: f64, t11915: f64, t11919: f64, t11923: f64, t11927: f64, t2312: f64, t2343: f64, t3247: f64, t9123: f64, t929: f64, t9579: f64) -> (f64, f64, f64, f64) {
    let t11928 = t1076 * t810;
    let t11930 = t2255 * t1123 * t11928;
    let t11934 = t3235 * t11464 * t875;
    let t11938 = t2345 * t11514 * t6287;
    let t11941 = t2343 * t11901 / 192.0_f64 - t11907 + t11911 + t11913 - 5.0_f64 / 128.0_f64 * t929 * t11915 + 5.0_f64 / 384.0_f64 * t929 * t11919 - t11923 + t11927 - t2312 * t11930 / 384.0_f64 + t9123 - t2343 * t11934 / 1536.0_f64 + t9579 - t3247 * t11938 / 128.0_f64;
    (t11930, t11934, t11938, t11941)
}
