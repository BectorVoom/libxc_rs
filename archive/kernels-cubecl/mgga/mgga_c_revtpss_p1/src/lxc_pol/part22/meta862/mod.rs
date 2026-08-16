//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta862 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3013;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta862<F: Float>(t14547: F, t14671: F, t14686: F, t50570: F, t2661: F, t2662: F, t2754: F, t4416: F, t14738: F, t2741: F, t10845: F, t14732: F, t4423: F, t853: F, t2749: F, t14718: F, t14872: F, t10777: F, t10779: F, t1548: F, t14931: F, t2724: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50573, t50577, t50579, t50581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3013::<F>(t14547, t14671, t14686, t50570, t2661, t2662, t2754, t4416, t14738, t2741, t10845, t14732);
        let (t50583, t50586, t50590, t50594, t50598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3014::<F>(t4423, t853, t2661, t2662, t2749, t14718, t14872, t10777, t10779, t1548, t2754, t14671, t14686, t14931, t2724);
    (t50573, t50577, t50579, t50581, t50583, t50586, t50590, t50594, t50598)
}
