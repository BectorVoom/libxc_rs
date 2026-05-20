//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta306<F: Float>(t1398: F, t281: F, t543: F, t68: F, t10139: F, t1357: F, t4078: F, t689: F, t1445: F, t3899: F, t10115: F, t562: F) -> (F, F, F, F, F, F, F) {
        let (t10142, t10143, t10150, t10151, t10153, t10154, t10157) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1743::<F>(t1398, t281, t543, t68, t10139, t1357, t4078, t689, t1445, t3899, t10115, t562);
    (t10142, t10143, t10150, t10151, t10153, t10154, t10157)
}
