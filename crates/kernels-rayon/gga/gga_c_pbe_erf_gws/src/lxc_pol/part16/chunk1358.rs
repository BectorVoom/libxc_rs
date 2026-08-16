//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1358/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1358(t54716: f64, t54719: f64, t54724: f64, t54730: f64, t14327: f64, t52036: f64, t52607: f64, t54690: f64, t54694: f64, t54697: f64, t54702: f64, t54707: f64, t54714: f64, t54722: f64, t54734: f64, t8654: f64) -> f64 {
    let t55983 = 7.0_f64 / 576.0_f64 * t54716;
    let t55984 = 35.0_f64 / 108.0_f64 * t54719;
    let t55986 = 119.0_f64 / 6912.0_f64 * t54724;
    let t55987 = 7.0_f64 / 576.0_f64 * t54730;
    let t55990 = t54690 / 192.0_f64 - t8654 * t14327 / 48.0_f64 - t54694 / 192.0_f64 + 7.0_f64 / 288.0_f64 * t52607 - t54697 / 96.0_f64 + t54702 / 384.0_f64 - t54707 / 384.0_f64 + t54714 / 12.0_f64 + t55983 - t55984 - t54722 / 24.0_f64 - t55986 + t55987 - t54734 / 8.0_f64 + 35.0_f64 / 108.0_f64 * t52036;
    t55990
}
