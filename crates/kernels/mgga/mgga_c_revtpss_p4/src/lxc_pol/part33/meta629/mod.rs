//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2074;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta629<F: Float>(t136: F, t2457: F, t7769: F, t93377: F, t4534: F, t689: F, t7014: F, t27303: F, t786: F, t789: F, t25296: F, t27216: F, t14991: F, t93261: F, t27213: F, t92843: F, t98815: F, t27291: F, t25431: F, t25411: F, t2453: F, t27212: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99211, t99212, t99216, t99219, t99222) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2074::<F>(t136, t2457, t7769, t93377, t4534, t689, t7014, t27303, t786, t789, t25296, t27216);
        let (t99228, t99231, t99234, t99243, t99245, t99257) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2075::<F>(t14991, t93261, t25296, t27213, t92843, t98815, t27291, t689, t25431, t25411, t2453, t27212);
    (t99211, t99212, t99216, t99219, t99222, t99228, t99231, t99234, t99243, t99245, t99257)
}
