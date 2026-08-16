//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta422(t3531: f64, t5202: f64, t300: f64, t5155: f64, t1198: f64, t3539: f64, t5192: f64, t12571: f64, t1765: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12382: f64, t16706: f64, t16708: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64, t422: f64, t12552: f64, t1756: f64, t12555: f64, t3497: f64, t1196: f64, t12367: f64, t448: f64, t1130: f64, t5060: f64, t1151: f64, t3428: f64, t5063: f64, t1719: f64, t3432: f64, t3436: f64, t12238: f64, t1733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16783, t16786, t16788, t16790, t16807) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1556(t3531, t5202, t300, t5155, t1198, t3539, t5192, t12571, t1765, t16710, t16712, t12297, t12299, t12301, t12303, t12382, t16706, t16708, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16809, t16814, t16831) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1557(t16807, t422, t12552, t1756, t12555, t3497, t1196, t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12367, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let (t16832, t16834, t16837, t16839, t16842, t16844) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1558(t16831, t448, t300, t1130, t5060, t1151, t3428, t5063, t1719, t3432, t3436, t12238, t1733);
    (t16783, t16786, t16788, t16790, t16809, t16814, t16832, t16834, t16837, t16839, t16842, t16844)
}
