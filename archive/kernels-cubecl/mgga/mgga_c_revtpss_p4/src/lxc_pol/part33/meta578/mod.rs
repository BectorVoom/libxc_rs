//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta578<F: Float>(t2411: F, t605: F, t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t25372: F, t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F, t25300: F, t9285: F, t25299: F, t7059: F, t9288: F, t7064: F, t25305: F, t136: F, t2457: F, t7082: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92790, t92819, t92838, t92843, t92858, t92861) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987::<F>(t2411, t605, t198, t206, t7086, t25373, t25392, t25386, t25372, t2435, t25352, t11015, t7018);
        let (t92864, t92870, t92871, t92873, t92875, t92894) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1988::<F>(t7048, t822, t25300, t9285, t25299, t7059, t9288, t7064, t25305, t136, t2457, t7082);
    (t92790, t92819, t92838, t92843, t92858, t92861, t92864, t92870, t92871, t92873, t92875, t92894)
}
