//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta933 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3063;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3064;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3065;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3066;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3067;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3068;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3069;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta933<F: Float>(t1121: F, t76397: F, t1120: F, t128: F, t24229: F, t689: F, t24233: F, t24241: F, t24249: F, t51957: F, t56246: F, t77513: F, t56254: F, t43888: F, t56176: F, t56184: F, t56229: F, t56236: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t81226, t81228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3063::<F>(t1121, t76397, t1120, t128);
        let t81230 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3064::<F>(t24229, t689);
        let t81232 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3065::<F>(t24233, t689);
        let t81234 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3066::<F>(t24241, t689);
        let t81236 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3067::<F>(t24249, t689);
        let t81242 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3068::<F>(t51957, t56246, t77513);
        let t81245 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3069::<F>(t51957, t56254, t77513);
        let t81250 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3070::<F>(t43888, t56176, t56184, t56229, t56236, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
    (t81226, t81228, t81230, t81232, t81234, t81236, t81242, t81245, t81250)
}
