//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2128;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2130;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta619<F: Float>(t2033: F, t3829: F, t2014: F, t7900: F, t28067: F, t95088: F, t14468: F, t30: F, t2: F, t2411: F, t580: F, t890: F, t27382: F, t198: F, t206: F, t7782: F, t892: F, t775: F, t25206: F, t1583: F, t2430: F, t25207: F, t1468: F, t14365: F, t1544: F, t2257: F, t205: F, t7086: F, t4433: F, t1940: F, t1963: F, t2403: F, t25198: F, t25208: F, t25449: F, t27158: F, t27160: F, t27169: F, t27364: F, t27368: F, t27395: F, t4541: F, t605: F, t7087: F, t7783: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98621, t98623, t98627, t98633) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2128::<F>(t2033, t3829, t2014, t7900, t28067, t95088, t14468, t30, t2, t2411, t580, t890);
        let (t98635, t98637, t98650, t98651) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129::<F>(t27382, t98633, t198, t206, t7782, t2, t892, t580, t775, t25206, t1583, t2430);
        let (t98652, t98659, t98662, t98669, t98674) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2130::<F>(t25207, t98651, t1468, t2411, t14365, t1544, t2257, t198, t205, t7086, t4433, t890);
        let t98678 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2131::<F>(t25207, t98674, t1940, t1963, t2403, t25198, t25206, t25208, t25449, t27158, t27160, t27169, t27364, t27368, t27395, t4541, t605, t7087, t7783, t98627, t98635, t98637, t98650, t98652, t98659, t98662, t98669);
    (t98621, t98623, t98635, t98637, t98650, t98651, t98669, t98674, t98678)
}
