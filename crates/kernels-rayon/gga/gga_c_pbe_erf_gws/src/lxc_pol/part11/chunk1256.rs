//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1256/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1256(t11564: f64, t11808: f64, t11557: f64, t11994: f64, t13539: f64, t2255: f64, t2277: f64, t3257: f64, t3780: f64, t45755: f64, t45793: f64, t49921: f64, t49928: f64, t49929: f64, t49931: f64, t49936: f64, t49943: f64, t9441: f64) -> (f64, f64) {
    let t49945 = t11564 * t11808 / 8.0_f64;
    let t49946 = -t49921 - 7.0_f64 / 384.0_f64 * t2277 * t3257 * t9441 * t11557 * t3780 - t49928 + t49929 + 7.0_f64 / 96.0_f64 * t45755 + t49931 + t49936 + 7.0_f64 / 576.0_f64 * t45793 - t2277 * t2255 * t11994 * t13539 / 256.0_f64 - t49943 - t49945;
    (t49945, t49946)
}
