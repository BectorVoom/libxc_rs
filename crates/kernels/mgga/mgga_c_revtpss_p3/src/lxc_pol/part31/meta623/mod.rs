//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2073;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta623<F: Float>(t25431: F, t99389: F, t1568: F, t786: F, t25410: F, t25413: F, t25375: F, t99365: F, t1579: F, t25392: F, t4481: F, t92921: F, t10073: F, t1958: F, t25390: F, t25305: F, t99380: F, t213: F, t27265: F, t2453: F, t2458: F, t7760: F, t25331: F, t27213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99391, t99403, t99404, t99406, t99412, t99414, t99420) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2073::<F>(t25431, t99389, t1568, t786, t25410, t25413, t25375, t99365, t1579, t25392, t4481, t92921);
        let (t99423, t99425, t99429, t99435, t99456) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2074::<F>(t10073, t1579, t1958, t25390, t25305, t99380, t213, t27265, t2453, t2458, t7760, t25331, t27213);
    (t99391, t99403, t99404, t99406, t99412, t99414, t99420, t99423, t99425, t99429, t99435, t99456)
}
