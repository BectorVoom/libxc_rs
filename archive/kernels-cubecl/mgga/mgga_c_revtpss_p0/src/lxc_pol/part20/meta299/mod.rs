//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1179;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1180;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1181;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta299<F: Float>(t1120: F, t12273: F, t128: F, t12287: F, t12277: F, t12292: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t1132: F) -> (F, F, F, F, F, F, F, F) {
        let (t12313, t12314) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1179::<F>(t1120, t12273, t128);
        let (t12316, t12317) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1180::<F>(t1120, t12287, t128);
        let (t12319, t12320) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1181::<F>(t1120, t12277, t128);
        let (t12322, t12323) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1182::<F>(t12292, t12296, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t1132);
    (t12313, t12314, t12316, t12317, t12319, t12320, t12322, t12323)
}
