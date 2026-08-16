//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk712;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk713;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk714;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk715;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta167<F: Float>(t198: F, t205: F, t1544: F, t262: F, t1583: F, t892: F, t2404: F, t2411: F, t1940: F, t207: F, t2403: F, t2621: F, t2628: F, t2632: F, t4316: F, t4343: F, t4394: F, t4396: F, t4397: F, t4400: F, t4405: F, t4406: F, t4537: F, t765: F, t775: F, t890: F, t4314: F, t2: F, t265: F, t580: F, t1593: F, t689: F, t1469: F, t2852: F, t606: F, t2850: F, t128: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4541, t4542, t4546, t4556, t4559) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk712::<F>(t198, t205, t1544, t262, t1583, t892, t2404, t2411, t1940, t207, t2403, t2621, t2628, t2632, t4316, t4343, t4394, t4396, t4397, t4400, t4405, t4406, t4537, t765, t775, t890);
        let t4560 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk713::<F>(t4314, t4559);
        let (t4568, t4571) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk714::<F>(t2, t265, t580, t1593, t689);
        let (t4573, t4574) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk715::<F>(t1469, t2852, t606);
        let (t4575, t4576) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk716::<F>(t2850, t4574, t128);
    (t4541, t4542, t4546, t4556, t4560, t4568, t4571, t4573, t4574, t4575, t4576)
}
