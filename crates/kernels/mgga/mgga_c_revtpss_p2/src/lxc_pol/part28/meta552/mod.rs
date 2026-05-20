//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2002;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta552<F: Float>(t10301: F, t25105: F, t116: F, t25168: F, t1962: F, t41154: F, t2411: F, t25435: F, t605: F, t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t268: F, t41040: F, t837: F, t25372: F, t25287: F, t786: F, t789: F, t2829: F, t689: F, t7014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92702, t92737, t92742, t92775, t92790, t92819) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2002::<F>(t10301, t25105, t116, t25168, t1962, t41154, t2411, t25435, t605, t198, t206, t7086);
        let (t92838, t92841, t92843, t92844, t92847, t92856) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2003::<F>(t25373, t25392, t25386, t268, t41040, t837, t25372, t25287, t786, t789, t2829, t689, t7014);
    (t92702, t92737, t92742, t92775, t92790, t92819, t92838, t92841, t92843, t92844, t92847, t92856)
}
