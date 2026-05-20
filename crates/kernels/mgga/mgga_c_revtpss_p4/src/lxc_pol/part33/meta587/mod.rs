//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2001;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta587<F: Float>(t10510: F, t25399: F, t10115: F, t1951: F, t7058: F, t92871: F, t1032: F, t11007: F, t233: F, t25372: F, t10509: F, t25377: F, t25375: F, t1957: F, t2718: F, t25386: F, t25331: F, t25365: F, t786: F, t860: F, t25410: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93273, t93276, t93278, t93280, t93281, t93285) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2001::<F>(t10510, t25399, t10115, t1951, t7058, t92871, t1032, t11007, t233, t25372, t10509, t25377);
        let (t93286, t93302, t93306, t93314, t93317, t93320, t93321) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2002::<F>(t25375, t93285, t1957, t2718, t25386, t25331, t25365, t25372, t93280, t786, t860, t25410);
    (t93273, t93276, t93278, t93281, t93285, t93286, t93302, t93306, t93314, t93317, t93320, t93321)
}
