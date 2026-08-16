//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta933 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta933<F: Float>(t12916: F, t17780: F, t5331: F, t1260: F, t45385: F, t12640: F, t17728: F, t489: F, t12744: F, t17350: F, t3781: F, t5219: F, t5330: F, t17743: F, t3718: F, t12881: F, t5391: F, t1222: F, t16720: F, t17471: F, t17753: F, t17755: F, t12800: F, t5378: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57336, t57344, t57348, t57378, t57382) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3163::<F>(t12916, t17780, t5331, t1260, t45385, t12640, t17728, t489, t12744, t17350, t3781, t5219, t5330);
        let (t57386, t57421, t57428, t57435, t57449) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164::<F>(t12916, t17743, t3718, t12881, t5391, t1222, t16720, t17471, t17753, t17755, t12800, t5378);
    (t57336, t57344, t57348, t57378, t57382, t57386, t57421, t57428, t57435, t57449)
}
