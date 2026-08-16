//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1978;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta567<F: Float>(t10309: F, t25105: F, t45972: F, t6957: F, t1962: F, t41154: F, t2411: F, t605: F, t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t25372: F, t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F, t25300: F, t9285: F, t25299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92687, t92690, t92742, t92790, t92819, t92837) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1978::<F>(t10309, t25105, t45972, t6957, t1962, t41154, t2411, t605, t198, t206, t7086, t25373, t25392);
        let (t92838, t92843, t92858, t92861, t92864, t92868, t92870) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979::<F>(t25386, t92837, t25372, t2435, t25352, t11015, t7018, t7048, t822, t25300, t9285, t25299);
    (t92687, t92690, t92742, t92790, t92819, t92838, t92843, t92858, t92861, t92864, t92868, t92870)
}
