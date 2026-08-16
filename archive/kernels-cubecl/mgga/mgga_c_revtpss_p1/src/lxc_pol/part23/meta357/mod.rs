//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1670;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta357<F: Float>(t14923: F, t4368: F, t2482: F, t2719: F, t814: F, t14671: F, t14686: F, t4366: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t2718: F) -> (F, F, F, F, F, F, F, F) {
        let (t14925, t14931) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1670::<F>(t14923, t4368, t2482, t2719, t814);
        let (t14933, t14934, t14946, t14948, t14951, t14961) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1671::<F>(t14671, t14686, t4366, t14931, t136, t1568, t2457, t2710, t2470, t4522, t874, t2718);
    (t14925, t14931, t14933, t14934, t14946, t14948, t14951, t14961)
}
