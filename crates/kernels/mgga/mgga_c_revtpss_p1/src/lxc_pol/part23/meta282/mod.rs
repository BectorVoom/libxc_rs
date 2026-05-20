//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta282<F: Float>(t136: F, t860: F, t2457: F, t2710: F, t10069: F, t2786: F, t10073: F, t10111: F, t22: F, t870: F, t10115: F, t253: F) -> (F, F, F, F, F, F) {
        let (t10914, t10916, t10923, t10925, t10939, t10948) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1507::<F>(t136, t860, t2457, t2710, t10069, t2786, t10073, t10111, t22, t870, t10115, t253);
    (t10914, t10916, t10923, t10925, t10939, t10948)
}
