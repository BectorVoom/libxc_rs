//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1312/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1312(t1144: f64, t14191: f64, t859: f64, t14180: f64, t4386: f64, t14949: f64, t9270: f64, t14943: f64, t14979: f64, t15025: f64, t2408: f64, t2409: f64, t3066: f64, t4385: f64, t50995: f64, t52183: f64, t53131: f64, t53134: f64, t53140: f64, t53152: f64, t53158: f64, t53166: f64, t53170: f64, t6781: f64, t6793: f64, t8734: f64) -> f64 {
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = 7.0_f64 / 72.0_f64 * t9270 * t14949;
    let t55003 = -t53131 / 768.0_f64 + t53134 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t50995 - t53140 / 192.0_f64 + t53152 / 192.0_f64 + t4385 * t54978 / 96.0_f64 - t53158 / 48.0_f64 - t53166 / 192.0_f64 + t6793 * t54984 / 24.0_f64 - 7.0_f64 / 288.0_f64 * t52183 + t2408 * t2409 * t6781 * t14979 / 24.0_f64 + t3066 * t2409 * t8734 * t15025 / 24.0_f64 + t53170 / 192.0_f64 - t54998 + t3066 * t2409 * t8734 * t14943 / 24.0_f64;
    t55003
}
