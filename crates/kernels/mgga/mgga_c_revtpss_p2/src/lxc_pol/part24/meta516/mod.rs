//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta516<F: Float>(t23958: F, t993: F, t225: F, t366: F, t20020: F, t4858: F, t1011: F, t140: F, t23877: F, t15823: F, t20029: F, t11710: F, t23899: F, t4892: F, t15987: F, t23503: F, t19773: F, t4845: F, t23868: F, t11922: F, t23930: F, t23903: F, t4899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t79862, t79863, t79864, t79874, t79881, t79892, t79938) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536::<F>(t23958, t993, t225, t366, t20020, t4858, t1011, t140, t23877, t15823, t20029, t11710, t23899, t4892);
        let (t79944, t79946, t79957, t80038, t80113) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537::<F>(t1011, t15987, t23503, t19773, t4845, t140, t23868, t11922, t23930, t4892, t11710, t23903, t4899);
    (t79862, t79863, t79864, t79874, t79881, t79892, t79938, t79944, t79946, t79957, t80038, t80113)
}
