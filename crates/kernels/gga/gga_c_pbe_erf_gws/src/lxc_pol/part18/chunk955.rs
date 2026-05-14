//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 955/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk955<F: Float>(t11901: F, t11907: F, t11911: F, t11913: F, t11915: F, t11919: F, t11923: F, t11927: F, t11930: F, t11934: F, t11938: F, t2312: F, t2343: F, t3247: F, t9123: F, t929: F, t9579: F) -> (F,) {
    let t11941 = t2343 * t11901 / 192.0 - t11907 + t11911 + t11913 - 5.0 / 128.0 * t929 * t11915 + 5.0 / 384.0 * t929 * t11919 - t11923 + t11927 - t2312 * t11930 / 384.0 + t9123 - t2343 * t11934 / 1536.0 + t9579 - t3247 * t11938 / 128.0;
    (t11941,)
}
