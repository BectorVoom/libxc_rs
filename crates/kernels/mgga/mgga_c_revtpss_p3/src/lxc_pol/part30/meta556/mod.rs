//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1995;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta556<F: Float>(t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t268: F, t41040: F, t837: F, t25372: F, t25287: F, t786: F, t789: F, t2829: F, t689: F, t7014: F, t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F, t25300: F, t9285: F, t25299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92819, t92838, t92841, t92843, t92844, t92847) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1995::<F>(t198, t206, t7086, t25373, t25392, t25386, t268, t41040, t837, t25372, t25287, t786, t789);
        let (t92856, t92858, t92861, t92864, t92868, t92870) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1996::<F>(t2829, t689, t7014, t2435, t25352, t11015, t7018, t7048, t822, t25300, t9285, t25299);
    (t92819, t92838, t92841, t92843, t92844, t92847, t92856, t92858, t92861, t92864, t92868, t92870)
}
