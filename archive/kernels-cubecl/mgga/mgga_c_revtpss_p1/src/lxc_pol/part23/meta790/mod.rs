//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2605;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta790<F: Float>(t10845: F, t18531: F, t18618: F, t2741: F, t18622: F, t6016: F, t853: F, t2661: F, t2662: F, t2749: F, t14718: F, t18637: F, t50583: F, t6035: F, t18408: F, t837: F, t18432: F, t40336: F, t5977: F, t10726: F, t10786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61572, t61574, t61576, t61579, t61582, t61612) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2605::<F>(t10845, t18531, t18618, t2741, t18622, t6016, t853, t2661, t2662, t2749, t14718, t18637);
        let (t61616, t61620, t61623, t61625, t61628) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2606::<F>(t2661, t2662, t50583, t6035, t18408, t837, t18432, t40336, t5977, t853, t10726, t10786);
    (t61572, t61574, t61576, t61579, t61582, t61612, t61616, t61620, t61623, t61625, t61628)
}
