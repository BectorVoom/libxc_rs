//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1270/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1270(t28173: f64, t39191: f64, t1076: f64, t45771: f64, t856: f64, t857: f64, t858: f64, t1109: f64, t11499: f64, t11551: f64, t13140: f64, t2306: f64, t2312: f64, t3257: f64, t3258: f64, t3780: f64, t3861: f64, t39174: f64, t46430: f64, t46436: f64, t46524: f64, t48998: f64, t902: f64, t905: f64) -> (f64, f64, f64, f64) {
    let t50237 = 455.0_f64 / 324.0_f64 * t28173;
    let t50247 = 35.0_f64 / 36.0_f64 * t39191;
    let t50253 = 7.0_f64 / 48.0_f64 * t45771 * t856 * t857 * t858 * t1076;
    let t50264 = 7.0_f64 / 288.0_f64 * t46430 + 7.0_f64 / 576.0_f64 * t46436 + t50237 + 119.0_f64 / 1152.0_f64 * t39174 + t902 * t905 * t3861 * t13140 / 512.0_f64 + t902 * t905 * t48998 * t2306 / 256.0_f64 - t50247 + 35.0_f64 / 48.0_f64 * t46524 + t50253 - t2312 * t3257 * t3258 * t11551 * t3780 / 16.0_f64 + t2312 * t3257 * t11499 * t11551 * t1109 / 16.0_f64;
    (t50237, t50247, t50253, t50264)
}
