//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta961 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta961<F: Float>(t39766: F, t49926: F, t49929: F, t1544: F, t2408: F, t49940: F, t18569: F, t2398: F, t39774: F, t14397: F, t14436: F, t18875: F, t2403: F, t39760: F, t39764: F, t39770: F, t39773: F) -> (F, F, F, F, F, F, F) {
        let (t61149, t61150, t61151, t61159, t61161, t61162, t61163) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3223::<F>(t39766, t49926, t49929, t1544, t2408, t49940, t18569, t2398, t39774, t14397, t14436, t18875, t2403, t39760, t39764, t39770, t39773);
    (t61149, t61150, t61151, t61159, t61161, t61162, t61163)
}
