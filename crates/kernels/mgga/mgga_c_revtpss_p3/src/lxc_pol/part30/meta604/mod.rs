//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta604<F: Float>(t12995: F, t26824: F, t12963: F, t7613: F, t12975: F, t2138: F, t12984: F, t12851: F, t2134: F, t3567: F, t8945: F, t26894: F, t29199: F, t3596: F, t37885: F, t2149: F, t1210: F, t26936: F, t3566: F, t13181: F, t3140: F, t1243: F, t2147: F, t44841: F, t7635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97279, t97281, t97283, t97288, t97296, t97304, t97308) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2065::<F>(t12995, t26824, t12963, t7613, t12975, t2138, t12984, t12851, t2134, t3567, t8945, t26894, t29199);
        let (t97313, t97318, t97343, t97348, t97358) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2066::<F>(t3596, t37885, t2149, t1210, t29199, t26936, t3566, t13181, t3140, t1243, t2147, t44841, t7635);
    (t97279, t97281, t97283, t97288, t97296, t97304, t97308, t97313, t97318, t97343, t97348, t97358)
}
