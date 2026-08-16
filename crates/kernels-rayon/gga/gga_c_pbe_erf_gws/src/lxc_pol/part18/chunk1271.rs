//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1271/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1271(t1114: f64, t332: f64, t3747: f64, t13793: f64, t14617: f64, t53229: f64, t3060: f64, t36200: f64, t36201: f64, t4155: f64, t52902: f64, t56061: f64, t56063: f64, t56067: f64, t56070: f64, t56074: f64, t56077: f64, t56080: f64, t56084: f64, t56093: f64, t56098: f64, t56101: f64, t827: f64) -> (f64, f64) {
    let t56104 = t1114 * t3747 * t332;
    let t56105 = t56104 * t13793;
    let t56107 = t53229 * t14617;
    let t56109 = t56061 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t56063 + t56067 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t56070 - t56074 / 1536.0_f64 - t56077 / 192.0_f64 - t56080 / 192.0_f64 - t827 * t56084 / 96.0_f64 - t52902 + t36200 * t36201 * t4155 * t3060 / 4.0_f64 - t56093 / 96.0_f64 - t56098 / 384.0_f64 - t56101 / 48.0_f64 - t56105 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t56107;
    (t56104, t56109)
}
