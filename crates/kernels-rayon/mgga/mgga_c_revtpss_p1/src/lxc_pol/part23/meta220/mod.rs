//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1295;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1296;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1297;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1298;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1299;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1300;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta220(t1045: f64, t373: f64, t6299: f64, t1042: f64, t1668: f64, t3155: f64, t3162: f64, t225: f64, t6235: f64, t366: f64, t1066: f64, t6100: f64, t247: f64, t3182: f64, t6092: f64, t6096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6301, t6302) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1295(t1045, t373, t6299, t1042);
        let t6305 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1296(t1668);
        let (t6306, t6307, t6308) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1297(t373, t6305, t3155, t1042);
        let (t6311, t6312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1298(t3162, t6306, t1042);
        let (t6317, t6318, t6323) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1299(t225, t6235, t366, t1066, t6100, t247);
        let t6327 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1300(t3182, t6092, t247);
        let t6331 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1301(t1066, t6096, t247);
    (t6301, t6302, t6305, t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6327, t6331)
}
