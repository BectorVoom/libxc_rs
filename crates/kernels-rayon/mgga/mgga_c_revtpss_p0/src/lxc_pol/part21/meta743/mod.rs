//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta743(t30: f64, t3834: f64, t580: f64, t2257: f64, t605: f64, t22: f64, t5552: f64, t588: f64, t13550: f64, t13553: f64, t1468: f64, t2: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t9335: f64, t9336: f64, t9344: f64, zeta_threshold: f64, t33: f64, t3842: f64, t1113: f64, t3351: f64, t5560: f64, t13565: f64, t13568: f64, t1711: f64, t3841: f64, t47040: f64, t516: f64, t5557: f64, t9350: f64, t9351: f64, t9357: f64, t162: f64, t189: f64, t512: f64, t46967: f64, t39419: f64, t39422: f64, t46297: f64, t46963: f64, t47753: f64, t47754: f64, t47758: f64, t47759: f64, t47760: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48165, t48168, t48174, t48177, t48187) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615(t30, t3834, t580, t2257, t605, t22, t5552, t588, t13550, t13553, t1468, t2, t3833, t47025, t513, t5549, t9335, t9336, t9344, zeta_threshold);
        let (t48192, t48195, t48201, t48204, t48214) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616(t33, t3842, t580, t1113, t3351, t22, t5560, t588, t13565, t13568, t1711, t2, t3841, t47040, t516, t5557, t9350, t9351, t9357, zeta_threshold);
        let (t48216, t48218, t48219, t48220) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617(t162, t48187, t48214, t189, t512, t46967, t39419, t39422, t46297, t46963, t47753, t47754, t47758, t47759, t47760, t48153, t48155, t48157, t48159, t48160);
    (t48165, t48168, t48174, t48177, t48192, t48195, t48201, t48204, t48216, t48218, t48219, t48220)
}
