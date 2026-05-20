//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta308<F: Float>(t10175: F, t3917: F, t3889: F, t566: F, t64: F, t843: F, t112: F, t2289: F, t666: F, t2341: F, t625: F, t2367: F) -> (F, F, F, F, F, F, F) {
        let (t10176, t10186, t10199, t10201, t10202, t10204, t10206) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1746::<F>(t10175, t3917, t3889, t566, t64, t843, t112, t2289, t666, t2341, t625, t2367);
    (t10176, t10186, t10199, t10201, t10202, t10204, t10206)
}
