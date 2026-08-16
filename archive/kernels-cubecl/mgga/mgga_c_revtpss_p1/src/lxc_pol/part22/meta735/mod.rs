//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta735<F: Float>(t10688: F, t243: F, t268: F, t40634: F, t2694: F, t9784: F, t10681: F, t2689: F, t16: F, t2236: F, t240: F, t236: F, t281: F, t39644: F) -> (F, F, F, F, F, F) {
        let (t40638, t40639, t40645, t40649, t40650, t40654) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2795::<F>(t10688, t243, t268, t40634, t2694, t9784, t10681, t2689, t16, t2236, t240, t236, t281, t39644);
    (t40638, t40639, t40645, t40649, t40650, t40654)
}
