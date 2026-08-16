//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2276;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta572(t3601: f64, t3603: f64, t17710: f64, t3720: f64, t13127: f64, t17708: f64, t471: f64, t17730: f64, t5046: f64, t12787: f64, t1260: f64, t5261: f64, t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t1261: f64, t1266: f64, t17721: f64, t17724: f64, t17729: f64, t17732: f64, t17736: f64, t17739: f64, t17744: f64, t17747: f64, t3718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17748, t17749, t17750, t17753, t17754, t17755, t17756, t17759, t17760, t17763) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2276(t3601, t3603, t17710, t3720, t13127, t17708, t471, t17730, t5046, t12787, t1260, t5261);
        let (t17769, t17772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2277(t3647, t5378, t247, t3634, t5056, t1261, t1266, t17721, t17724, t17729, t17732, t17736, t17739, t17744, t17747, t17750, t17753, t17756, t17760, t17763, t3718);
    (t17748, t17749, t17750, t17753, t17754, t17755, t17756, t17759, t17760, t17763, t17769, t17772)
}
