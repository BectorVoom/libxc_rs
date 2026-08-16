//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1374/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1374(t14185: f64, t15483: f64, t15543: f64, t2376: f64, t2408: f64, t2409: f64, t335: f64, t338: f64, t3717: f64, t4110: f64, t53915: f64, t55660: f64, t55672: f64, t57311: f64, t57319: f64, t57324: f64, t57326: f64, t57330: f64, t57332: f64, t57334: f64, t57338: f64, t6781: f64, t892: f64, t9283: f64, t9926: f64) -> f64 {
    let t58516 = -t335 * t338 * t892 * t15483 / 96.0_f64 - t55660 + 5.0_f64 / 384.0_f64 * t57311 + t57319 / 1536.0_f64 - t2408 * t9283 * t14185 * t9926 / 12.0_f64 + t57324 / 384.0_f64 - 7.0_f64 / 72.0_f64 * t57326 + t55672 + t57330 / 384.0_f64 + t57332 / 12.0_f64 + t57334 / 4.0_f64 - t53915 - 7.0_f64 / 288.0_f64 * t57338 + t2408 * t2409 * t6781 * t15543 / 48.0_f64 + t2408 * t2409 * t2376 * t4110 * t3717 / 48.0_f64;
    t58516
}
