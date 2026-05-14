//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1092/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1092<F: Float>(t1076: F, t1123: F, t11499: F, t1153: F, t13269: F, t2118: F, t2253: F, t2255: F, t2277: F, t3257: F, t343: F, t3757: F, t3781: F, t49491: F, t49534: F, t49745: F, t49761: F, t49763: F, t49765: F, t49767: F, t49768: F, t49773: F, t6275: F, t6637: F, t9499: F) -> (F,) {
    let t49777 = -t2253 * t2255 * t3781 * t13269 / 128.0 + t49745 + t6637 * t9499 * t2118 * t49534 / 128.0 - 7.0 / 384.0 * t2277 * t3257 * t11499 * t3757 * t1076 - t2253 * t2255 * t1123 * t49491 * t343 / 128.0 - t49761 - t49763 + t49765 + t49767 + t6275 * t1153 * t49768 / 16.0 + t6275 * t1153 * t49773 / 16.0;
    (t49777,)
}
