//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2370;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta617<F: Float>(t10871: F, t2645: F, t234: F, t39545: F, t685: F, t875: F, t2760: F, t2783: F, t786: F, t2801: F, t10069: F, t10920: F, t231: F, t2782: F, t39709: F, t10910: F, t233: F, t689: F, t869: F, t2778: F, t39515: F, t39501: F, t871: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40284, t40294, t40297, t40298, t40303) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2370::<F>(t10871, t2645, t234, t39545, t685, t875, t2760, t2783, t786, t2801, t10069, t10920);
        let (t40307, t40311, t40314, t40316) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2371::<F>(t231, t2782, t2783, t39709, t10910, t233, t689, t869, t2778, t39515, t39501, t871);
    (t40284, t40294, t40297, t40298, t40303, t40307, t40311, t40314, t40316)
}
