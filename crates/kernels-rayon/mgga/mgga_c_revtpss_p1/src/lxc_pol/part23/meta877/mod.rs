//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta877 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta877(t14110: f64, t49471: f64, t136: f64, t2457: f64, t47480: f64, t6895: f64, t22414: f64, t686: f64, t72: f64, t9680: f64, t22386: f64, t3915: f64, t49503: f64, t5722: f64, t213: f64, t22307: f64, t1358: f64, t2439: f64, t6888: f64, t785: f64, t1357: f64, t22387: f64, t689: f64, t3899: f64, t6896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74763, t74770, t74782, t74794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782(t14110, t49471, t136, t2457, t47480, t6895, t22414, t686, t72, t9680, t22386, t3915);
        let (t74797, t74802, t74807, t74810, t74813) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2783(t49503, t5722, t213, t22307, t1358, t2439, t6888, t785, t1357, t22387, t689, t3899, t6896);
    (t74763, t74770, t74782, t74794, t74797, t74802, t74807, t74810, t74813)
}
