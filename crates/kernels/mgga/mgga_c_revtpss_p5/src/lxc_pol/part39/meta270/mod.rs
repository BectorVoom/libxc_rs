//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1003;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta270<F: Float>(t555: F, t9646: F, t1358: F, t22: F, t1425: F, t225: F, t3907: F, t9285: F, t3906: F, t1357: F, t4132: F, t689: F, t4131: F, t676: F, t123: F, t3915: F, t2453: F, t3914: F, t1444: F, t2438: F, t138: F, t4075: F, t556: F, t786: F, t4077: F, t2434: F, t1359: F, t9292: F, t1363: F, t9288: F, t1362: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9650, t9657, t9666, t9668) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1002::<F>(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t1357, t4132, t689);
        let (t9672, t9674, t9677, t9680) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1003::<F>(t4131, t676, t123, t3915, t2453, t3914, t1444, t2438, t138, t4075, t556, t786);
        let (t9683, t9687, t9691, t9694) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1004::<F>(t4077, t676, t123, t9680, t1444, t2434, t3915, t1359, t9292, t1363, t9288, t1362);
    (t9650, t9657, t9666, t9668, t9672, t9674, t9677, t9680, t9683, t9687, t9691, t9694)
}
