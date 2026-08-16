//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta569<F: Float>(t233: F, t93279: F, t25372: F, t10996: F, t25377: F, t10509: F, t25375: F, t25296: F, t25365: F, t1957: F, t2718: F, t25386: F, t25418: F, t689: F, t25331: F, t25325: F, t686: F, t72: F, t25387: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93280, t93281, t93282, t93283, t93285, t93286, t93297, t93301, t93302) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2017::<F>(t233, t93279, t25372, t10996, t25377, t10509, t25375, t25296, t25365, t1957, t2718, t25386);
        let (t93304, t93306, t93311, t93312, t93315, t93317) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2018::<F>(t25418, t689, t93302, t25331, t25365, t25325, t686, t72, t25387, t25372, t93301, t25386, t93280);
    (t93281, t93282, t93283, t93285, t93286, t93297, t93304, t93306, t93311, t93312, t93315, t93317)
}
