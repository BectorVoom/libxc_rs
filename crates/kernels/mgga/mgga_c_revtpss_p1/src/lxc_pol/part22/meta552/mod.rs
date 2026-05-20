//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta552<F: Float>(t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t1261: F, t1266: F, t17721: F, t17724: F, t17729: F, t17732: F, t17736: F, t17739: F, t17744: F, t17747: F, t17750: F, t17753: F, t17756: F, t17760: F, t17763: F, t3718: F) -> (F, F, F, F) {
        let (t17767, t17769, t17771, t17772) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2376::<F>(t3647, t5378, t247, t3634, t5056, t1261, t1266, t17721, t17724, t17729, t17732, t17736, t17739, t17744, t17747, t17750, t17753, t17756, t17760, t17763, t3718);
    (t17767, t17769, t17771, t17772)
}
