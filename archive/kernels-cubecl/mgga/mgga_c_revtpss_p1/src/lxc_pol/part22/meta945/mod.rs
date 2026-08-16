//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta945 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta945<F: Float>(t17550: F, t372: F, t17352: F, t3153: F, t3623: F, t53667: F, t45619: F, t3666: F, t5390: F, t17794: F, t1261: F, t17203: F, t3172: F) -> (F, F, F, F, F, F, F) {
        let (t58899, t58909, t58919, t58920, t58927, t58960, t58975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3182::<F>(t17550, t372, t17352, t3153, t3623, t53667, t45619, t3666, t5390, t17794, t1261, t17203, t3172);
    (t58899, t58909, t58919, t58920, t58927, t58960, t58975)
}
