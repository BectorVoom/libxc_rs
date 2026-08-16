//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta562<F: Float>(t2438: F, t837: F, t93172: F, t93170: F, t25305: F, t92894: F, t786: F, t92889: F, t7060: F, t2434: F, t25377: F, t25431: F) -> (F, F, F, F, F, F) {
        let (t93174, t93175, t93177, t93180, t93183, t93184) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2020::<F>(t2438, t837, t93172, t93170, t25305, t92894, t786, t92889, t7060, t2434, t25377, t25431);
    (t93174, t93175, t93177, t93180, t93183, t93184)
}
