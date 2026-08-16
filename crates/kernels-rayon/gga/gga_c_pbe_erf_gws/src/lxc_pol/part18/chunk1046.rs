//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1046/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1046(t11737: f64, t2300: f64, t904: f64, t11680: f64, t11685: f64, t11689: f64, t11695: f64, t11699: f64, t11701: f64, t11734: f64, t2277: f64, t2312: f64, t8960: f64, t8969: f64, t8971: f64, t8973: f64, t914: f64, t929: f64, t9498: f64) -> (f64, f64) {
    let t11739 = t2300 * t904 * t11737;
    let t11742 = t8960 - t2312 * t11680 / 192.0_f64 - t2312 * t11685 / 192.0_f64 - t2277 * t11689 / 384.0_f64 - t11695 + t11699 - t8969 - t2312 * t11701 / 384.0_f64 + t8971 - t914 * t11734 / 1536.0_f64 + 5.0_f64 / 768.0_f64 * t929 * t11739 + t9498 + t8973;
    (t11739, t11742)
}
