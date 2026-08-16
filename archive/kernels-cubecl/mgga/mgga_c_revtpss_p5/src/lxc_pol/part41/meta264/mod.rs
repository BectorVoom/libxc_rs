//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1006;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1007;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta264<F: Float>(t1333: F, t3860: F, t30: F, t513: F, t33: F, t516: F, t2435: F, t3900: F, t3896: F, t9303: F, t1419: F, t785: F, t1358: F, t2439: F, t784: F, t209: F, t555: F, t22: F, t1425: F, t225: F, t3907: F, t9285: F, t3906: F, t2453: F, t3914: F, t1444: F, t2438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9597, t9605, t9617, t9632, t9639, t9640) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1006::<F>(t1333, t3860, t30, t513, t33, t516, t2435, t3900, t3896, t9303, t1419, t785);
        let (t9642, t9646) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1007::<F>(t1358, t9640, t2439, t784, t209);
        let (t9650, t9657, t9666, t9674, t9675) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1008::<F>(t555, t9646, t1358, t22, t1425, t225, t3907, t9285, t3906, t2453, t3914, t1444, t2438);
    (t9597, t9605, t9617, t9632, t9639, t9642, t9646, t9650, t9657, t9666, t9674, t9675)
}
