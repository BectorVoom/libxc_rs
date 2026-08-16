//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1401/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1401(t15537: f64, t22493: f64, t1144: f64, t15082: f64, t2408: f64, t335: f64, t338: f64, t35193: f64, t3742: f64, t4083: f64, t52191: f64, t54719: f64, t54724: f64, t54737: f64, t55983: f64, t55987: f64, t57719: f64, t57731: f64, t57740: f64, t57745: f64, t57747: f64, t57755: f64, t57764: f64, t9283: f64) -> f64 {
    let t58951 = t22493 * t15537;
    let t58962 = t57719 / 192.0_f64 + t57731 / 1536.0_f64 - t2408 * t9283 * t52191 * t3742 / 12.0_f64 - t57740 / 1536.0_f64 + t57745 / 768.0_f64 - t57747 / 8.0_f64 + t57755 / 96.0_f64 + t55983 - 7.0_f64 / 288.0_f64 * t58951 - t35193 * t4083 / 96.0_f64 - 35.0_f64 / 54.0_f64 * t54719 - t335 * t338 * t1144 * t15082 / 48.0_f64 - 119.0_f64 / 3456.0_f64 * t54724 - t57764 / 1536.0_f64 + t55987 + t54737;
    t58962
}
