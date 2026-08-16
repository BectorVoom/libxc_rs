//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta877 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta877<F: Float>(t14110: F, t49471: F, t136: F, t2457: F, t47480: F, t6895: F, t22414: F, t686: F, t72: F, t9680: F, t22386: F, t3915: F, t49503: F, t5722: F, t213: F, t22307: F, t1358: F, t2439: F, t6888: F, t785: F, t1357: F, t22387: F, t689: F, t3899: F, t6896: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74763, t74770, t74782, t74794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782::<F>(t14110, t49471, t136, t2457, t47480, t6895, t22414, t686, t72, t9680, t22386, t3915);
        let (t74797, t74802, t74807, t74810, t74813) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2783::<F>(t49503, t5722, t213, t22307, t1358, t2439, t6888, t785, t1357, t22387, t689, t3899, t6896);
    (t74763, t74770, t74782, t74794, t74797, t74802, t74807, t74810, t74813)
}
