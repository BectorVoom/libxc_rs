//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2076;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta630<F: Float>(t25301: F, t99257: F, t25410: F, t7774: F, t93240: F, t1032: F, t4469: F, t867: F, t786: F, t7060: F, t7760: F, t2467: F, t10073: F, t25403: F, t27198: F, t1955: F, t2471: F, t27202: F, t15003: F, t93194: F, t27266: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99258, t99261, t99270, t99271, t99274, t99285, t99287) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2076::<F>(t25301, t99257, t25410, t7774, t93240, t1032, t4469, t867, t786, t7060, t7760, t2467);
        let (t99297, t99303, t99307, t99313, t99321) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2077::<F>(t10073, t25403, t27198, t1955, t99270, t2471, t27202, t15003, t93194, t27266, t686, t72);
    (t99258, t99261, t99271, t99274, t99285, t99287, t99297, t99303, t99307, t99313, t99321)
}
