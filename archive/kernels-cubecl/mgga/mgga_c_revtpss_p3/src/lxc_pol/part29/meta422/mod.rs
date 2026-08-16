//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta422<F: Float>(t3531: F, t5202: F, t300: F, t5155: F, t1198: F, t3539: F, t5192: F, t12571: F, t1765: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12382: F, t16706: F, t16708: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F, t422: F, t12552: F, t1756: F, t12555: F, t3497: F, t1196: F, t12367: F, t448: F, t1130: F, t5060: F, t1151: F, t3428: F, t5063: F, t1719: F, t3432: F, t3436: F, t12238: F, t1733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16783, t16786, t16788, t16790, t16807) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1556::<F>(t3531, t5202, t300, t5155, t1198, t3539, t5192, t12571, t1765, t16710, t16712, t12297, t12299, t12301, t12303, t12382, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16809, t16814, t16831) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1557::<F>(t16807, t422, t12552, t1756, t12555, t3497, t1196, t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12367, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16832, t16834, t16837, t16839, t16842, t16844) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1558::<F>(t16831, t448, t300, t1130, t5060, t1151, t3428, t5063, t1719, t3432, t3436, t12238, t1733);
    (t16783, t16786, t16788, t16790, t16809, t16814, t16832, t16834, t16837, t16839, t16842, t16844)
}
