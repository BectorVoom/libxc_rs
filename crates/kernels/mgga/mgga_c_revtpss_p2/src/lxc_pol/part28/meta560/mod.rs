//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta560<F: Float>(t10073: F, t25402: F, t7048: F, t7056: F, t233: F, t41077: F, t25348: F, t689: F, t25411: F, t1955: F, t92888: F, t9646: F, t1949: F, t22: F, t1954: F, t39643: F, t25296: F, t25310: F, t25313: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t93116, t93118, t93123, t93124, t93126, t93134) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2017::<F>(t10073, t25402, t7048, t7056, t233, t41077, t25348, t689, t25411, t1955, t92888, t9646);
        let (t93138, t93139, t93142, t93143, t93146) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2018::<F>(t1949, t22, t25402, t93134, t1954, t39643, t7056, t25296, t25310, t25313, t686, t72);
    (t93116, t93118, t93123, t93124, t93126, t93138, t93139, t93142, t93143, t93146)
}
