//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta459<F: Float>(t13625: F, t8717: F, t25082: F, t1450: F, t3889: F, t7237: F, t2014: F, t7235: F, t7316: F, t1931: F, t2327: F, t10301: F, t6957: F) -> (F, F, F, F, F, F, F, F) {
        let (t25083, t25085, t25089, t25090, t25092, t25095, t25096, t25099) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1756::<F>(t13625, t8717, t25082, t1450, t3889, t7237, t2014, t7235, t7316, t1931, t2327, t10301, t6957);
    (t25083, t25085, t25089, t25090, t25092, t25095, t25096, t25099)
}
