//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2275;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2276;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2277;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta513<F: Float>(t1280: F, t16771: F, t1774: F, t3584: F, t16641: F, t16645: F, t16647: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16675: F, t16679: F, t16681: F, t16684: F, t16687: F, t16690: F, t3531: F, t5202: F, t300: F, t5155: F, t1198: F, t3539: F, t5192: F, t12571: F, t1765: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12382: F, t16706: F, t16708: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16772, t16775) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2275::<F>(t1280, t16771, t1774, t3584);
        let (t16776, t16781) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2276::<F>(t1280, t16775, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690);
        let (t16783, t16784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2277::<F>(t3531, t5202, t300, t5155);
        let (t16786, t16788, t16790, t16797, t16798, t16807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2278::<F>(t1198, t16784, t3539, t5192, t12571, t1765, t16710, t16712, t12297, t12299, t12301, t12303, t12382, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16772, t16775, t16776, t16781, t16783, t16784, t16786, t16788, t16790, t16797, t16798, t16807)
}
