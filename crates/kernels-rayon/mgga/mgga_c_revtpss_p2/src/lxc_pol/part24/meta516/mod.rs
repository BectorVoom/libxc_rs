//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta516(t23958: f64, t993: f64, t225: f64, t366: f64, t20020: f64, t4858: f64, t1011: f64, t140: f64, t23877: f64, t15823: f64, t20029: f64, t11710: f64, t23899: f64, t4892: f64, t15987: f64, t23503: f64, t19773: f64, t4845: f64, t23868: f64, t11922: f64, t23930: f64, t23903: f64, t4899: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79862, t79863, t79864, t79874, t79881, t79892, t79938) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536(t23958, t993, t225, t366, t20020, t4858, t1011, t140, t23877, t15823, t20029, t11710, t23899, t4892);
        let (t79944, t79946, t79957, t80038, t80113) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537(t1011, t15987, t23503, t19773, t4845, t140, t23868, t11922, t23930, t4892, t11710, t23903, t4899);
    (t79862, t79863, t79864, t79874, t79881, t79892, t79938, t79944, t79946, t79957, t80038, t80113)
}
