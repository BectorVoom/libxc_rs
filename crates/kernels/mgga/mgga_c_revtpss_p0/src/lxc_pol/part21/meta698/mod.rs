//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta698<F: Float>(t1214: F, t17703: F, t1243: F, t42859: F, t460: F, t1204: F, t13126: F, t12722: F, t3566: F, t5462: F, t5477: F, t1209: F, t1284: F, t3727: F) -> (F, F, F, F, F, F, F, F) {
        let (t45796, t45832, t45833, t45846, t45852, t45859, t45863, t45868) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2520::<F>(t1214, t17703, t1243, t42859, t460, t1204, t13126, t12722, t3566, t5462, t5477, t1209, t1284, t3727);
    (t45796, t45832, t45833, t45846, t45852, t45859, t45863, t45868)
}
