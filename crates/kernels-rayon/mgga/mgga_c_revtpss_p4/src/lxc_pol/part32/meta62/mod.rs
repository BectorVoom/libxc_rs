//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta62 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk385;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk386;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk387;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk388;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta62(t1128: f64, t1153: f64, t1156: f64, t1161: f64, t1170: f64, t1176: f64, t1180: f64, t1189: f64, t300: f64, t435: f64, t439: f64, t1179: f64, t1187: f64, t1188: f64, t1118: f64, t1124: f64, t459: f64, t458: f64, t456: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1193, t1195, t1196) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk385(t1128, t1153, t1156, t1161, t1170, t1176, t1180, t1189, t300, t435, t439);
        let (t1198, t1200, t1201, t1203, t1204) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk386(t1179, t1187, t1188, t1196, t1118, t1124, t459);
        let (t1207, t1208) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk387(t458);
        let t1209 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk388(t1208, t456);
        let t1210 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk389(t1209, t487);
    (t1193, t1195, t1196, t1198, t1200, t1201, t1203, t1204, t1207, t1208, t1209, t1210)
}
