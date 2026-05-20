//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta526<F: Float>(t12772: F, t24786: F, t3625: F, t17572: F, t21188: F, t13052: F, t24667: F, t3172: F, t12916: F, t24705: F, t3718: F, t1222: F, t17240: F, t24244: F, t24648: F, t3711: F, t1261: F, t24228: F, t247: F, t44895: F, t20820: F, t5265: F, t20851: F, t5362: F, t21101: F, t5273: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t83435, t83462, t83485, t83490, t83504) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1558::<F>(t12772, t24786, t3625, t17572, t21188, t13052, t24667, t3172, t12916, t24705, t3718, t1222, t17240, t24244);
        let (t83539, t83558, t83580, t83584, t83603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1559::<F>(t24648, t3172, t3711, t1261, t24228, t247, t44895, t20820, t5265, t20851, t5362, t21101, t5273);
    (t83435, t83462, t83485, t83490, t83504, t83539, t83558, t83580, t83584, t83603)
}
