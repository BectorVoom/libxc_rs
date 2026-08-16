//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1185/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1185(t21027: f64, t343: f64, t20726: f64, t6271: f64, t6484: f64, t6573: f64, t814: f64, t20344: f64, t2157: f64, t19737: f64, t20305: f64, t20964: f64, t20969: f64, t2113: f64, t2171: f64, t2255: f64, t2277: f64, t2312: f64, t2343: f64, t2345: f64, t6350: f64, t6390: f64, t6664: f64, t6685: f64, t6686: f64, t821: f64, t851: f64, t902: f64, t904: f64, t905: f64, t914: f64, t916: f64) -> (f64, f64, f64, f64) {
    let t21028 = t21027 * t343;
    let t21033 = t20726 * t343;
    let t21038 = t6484 * t6271;
    let t21039 = 7.0_f64 / 12.0_f64 * t21038;
    let t21040 = t6573 * t814;
    let t21053 = t20344 * t2157;
    let t21062 = t20964 + t20969 + t2343 * t2345 * t20305 * t2171 / 96.0_f64 - t914 * t916 * t904 * t21028 / 1536.0_f64 - t914 * t916 * t904 * t21033 / 1536.0_f64 - t21039 + t2277 * t2255 * t6664 * t21040 / 256.0_f64 + 3.0_f64 / 128.0_f64 * t6685 * t2255 * t2113 * t6686 + t2312 * t2255 * t6350 * t6390 / 48.0_f64 + 3.0_f64 / 128.0_f64 * t6685 * t2255 * t851 * t21053 + t902 * t905 * t821 * t19737 / 1536.0_f64;
    (t21028, t21033, t21039, t21062)
}
