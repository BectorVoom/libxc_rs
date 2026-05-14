//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1110/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1110<F: Float>(t28173: F, t39191: F, t1076: F, t45771: F, t856: F, t857: F, t858: F, t1109: F, t11499: F, t11551: F, t13140: F, t2306: F, t2312: F, t3257: F, t3258: F, t3780: F, t3861: F, t39174: F, t46430: F, t46436: F, t46524: F, t48998: F, t902: F, t905: F) -> (F, F, F, F) {
    let t50237 = 455.0 / 324.0 * t28173;
    let t50247 = 35.0 / 36.0 * t39191;
    let t50253 = 7.0 / 48.0 * t45771 * t856 * t857 * t858 * t1076;
    let t50264 = 7.0 / 288.0 * t46430 + 7.0 / 576.0 * t46436 + t50237 + 119.0 / 1152.0 * t39174 + t902 * t905 * t3861 * t13140 / 512.0 + t902 * t905 * t48998 * t2306 / 256.0 - t50247 + 35.0 / 48.0 * t46524 + t50253 - t2312 * t3257 * t3258 * t11551 * t3780 / 16.0 + t2312 * t3257 * t11499 * t11551 * t1109 / 16.0;
    (t50237, t50247, t50253, t50264)
}
