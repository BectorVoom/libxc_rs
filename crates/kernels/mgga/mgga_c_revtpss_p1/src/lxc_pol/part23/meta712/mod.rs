//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2470;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta712<F: Float>(t14238: F, t2453: F, t10142: F, t10073: F, t14231: F, t10139: F, t14219: F, t9285: F, t14215: F, t2470: F, t4101: F, t14220: F, t46495: F, t4086: F, t5710: F, t786: F, t10014: F, t14242: F, t14225: F, t1892: F, t5744: F, t136: F, t2457: F, t3964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48007, t48009, t48029, t48036, t48040, t48041) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2470::<F>(t14238, t2453, t10142, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101, t14220, t46495);
        let (t48042, t48048, t48080, t48082, t48083, t48084, t48089) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2471::<F>(t48041, t4086, t5710, t786, t10014, t14242, t10073, t14225, t1892, t5744, t136, t2457, t3964);
    (t48007, t48009, t48029, t48036, t48040, t48042, t48048, t48080, t48082, t48083, t48084, t48089)
}
