//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta289<F: Float>(t136: F, t860: F, t2457: F, t2710: F, t10069: F, t2786: F, t10073: F, t10111: F, t22: F, t870: F, t10115: F, t253: F, t2777: F, t2789: F, t2439: F, t2435: F, t2790: F, t2778: F, t9303: F, t871: F, t9292: F, t251: F, t9646: F, t780: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10916, t10923, t10925, t10939, t10948) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1048::<F>(t136, t860, t2457, t2710, t10069, t2786, t10073, t10111, t22, t870, t10115, t253);
        let (t10964, t10966, t10969, t10971, t10981, t10982) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1049::<F>(t2777, t2789, t2439, t2435, t2790, t2778, t9303, t871, t9292, t251, t9646, t22, t780);
    (t10916, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t10981, t10982)
}
