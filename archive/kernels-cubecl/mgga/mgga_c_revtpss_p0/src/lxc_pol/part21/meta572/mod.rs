//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2276;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta572<F: Float>(t3601: F, t3603: F, t17710: F, t3720: F, t13127: F, t17708: F, t471: F, t17730: F, t5046: F, t12787: F, t1260: F, t5261: F, t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t1261: F, t1266: F, t17721: F, t17724: F, t17729: F, t17732: F, t17736: F, t17739: F, t17744: F, t17747: F, t3718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17748, t17749, t17750, t17753, t17754, t17755, t17756, t17759, t17760, t17763) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2276::<F>(t3601, t3603, t17710, t3720, t13127, t17708, t471, t17730, t5046, t12787, t1260, t5261);
        let (t17769, t17772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2277::<F>(t3647, t5378, t247, t3634, t5056, t1261, t1266, t17721, t17724, t17729, t17732, t17736, t17739, t17744, t17747, t17750, t17753, t17756, t17760, t17763, t3718);
    (t17748, t17749, t17750, t17753, t17754, t17755, t17756, t17759, t17760, t17763, t17769, t17772)
}
