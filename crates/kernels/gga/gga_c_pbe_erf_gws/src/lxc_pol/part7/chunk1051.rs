//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1051/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1051<F: Float>(t21027: F, t343: F, t20726: F, t6271: F, t6484: F, t6573: F, t814: F, t20344: F, t2157: F, t19737: F, t20305: F, t20964: F, t20969: F, t2113: F, t2171: F, t2255: F, t2277: F, t2312: F, t2343: F, t2345: F, t6350: F, t6390: F, t6664: F, t6685: F, t6686: F, t821: F, t851: F, t902: F, t904: F, t905: F, t914: F, t916: F) -> (F, F, F, F) {
    let t21028 = t21027 * t343;
    let t21033 = t20726 * t343;
    let t21038 = t6484 * t6271;
    let t21039 = 7.0 / 12.0 * t21038;
    let t21040 = t6573 * t814;
    let t21053 = t20344 * t2157;
    let t21062 = t20964 + t20969 + t2343 * t2345 * t20305 * t2171 / 96.0 - t914 * t916 * t904 * t21028 / 1536.0 - t914 * t916 * t904 * t21033 / 1536.0 - t21039 + t2277 * t2255 * t6664 * t21040 / 256.0 + 3.0 / 128.0 * t6685 * t2255 * t2113 * t6686 + t2312 * t2255 * t6350 * t6390 / 48.0 + 3.0 / 128.0 * t6685 * t2255 * t851 * t21053 + t902 * t905 * t821 * t19737 / 1536.0;
    (t21028, t21033, t21039, t21062)
}
