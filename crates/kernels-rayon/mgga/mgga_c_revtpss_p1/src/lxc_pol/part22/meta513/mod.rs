//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2275;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2276;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2277;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta513(t1280: f64, t16771: f64, t1774: f64, t3584: f64, t16641: f64, t16645: f64, t16647: f64, t16649: f64, t16651: f64, t16654: f64, t16657: f64, t16660: f64, t16664: f64, t16667: f64, t16671: f64, t16675: f64, t16679: f64, t16681: f64, t16684: f64, t16687: f64, t16690: f64, t3531: f64, t5202: f64, t300: f64, t5155: f64, t1198: f64, t3539: f64, t5192: f64, t12571: f64, t1765: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12382: f64, t16706: f64, t16708: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16772, t16775) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2275(t1280, t16771, t1774, t3584);
        let (t16776, t16781) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2276(t1280, t16775, t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16671, t16675, t16679, t16681, t16684, t16687, t16690);
        let (t16783, t16784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2277(t3531, t5202, t300, t5155);
        let (t16786, t16788, t16790, t16797, t16798, t16807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2278(t1198, t16784, t3539, t5192, t12571, t1765, t16710, t16712, t12297, t12299, t12301, t12303, t12382, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16772, t16775, t16776, t16781, t16783, t16784, t16786, t16788, t16790, t16797, t16798, t16807)
}
