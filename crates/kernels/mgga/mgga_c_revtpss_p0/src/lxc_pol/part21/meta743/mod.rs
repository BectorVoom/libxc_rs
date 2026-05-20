//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta743<F: Float>(t30: F, t3834: F, t580: F, t2257: F, t605: F, t22: F, t5552: F, t588: F, t13550: F, t13553: F, t1468: F, t2: F, t3833: F, t47025: F, t513: F, t5549: F, t9335: F, t9336: F, t9344: F, zeta_threshold: F, t33: F, t3842: F, t1113: F, t3351: F, t5560: F, t13565: F, t13568: F, t1711: F, t3841: F, t47040: F, t516: F, t5557: F, t9350: F, t9351: F, t9357: F, t162: F, t189: F, t512: F, t46967: F, t39419: F, t39422: F, t46297: F, t46963: F, t47753: F, t47754: F, t47758: F, t47759: F, t47760: F, t48153: F, t48155: F, t48157: F, t48159: F, t48160: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48165, t48168, t48174, t48177, t48187) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2615::<F>(t30, t3834, t580, t2257, t605, t22, t5552, t588, t13550, t13553, t1468, t2, t3833, t47025, t513, t5549, t9335, t9336, t9344, zeta_threshold);
        let (t48192, t48195, t48201, t48204, t48214) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2616::<F>(t33, t3842, t580, t1113, t3351, t22, t5560, t588, t13565, t13568, t1711, t2, t3841, t47040, t516, t5557, t9350, t9351, t9357, zeta_threshold);
        let (t48216, t48218, t48219, t48220) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2617::<F>(t162, t48187, t48214, t189, t512, t46967, t39419, t39422, t46297, t46963, t47753, t47754, t47758, t47759, t47760, t48153, t48155, t48157, t48159, t48160);
    (t48165, t48168, t48174, t48177, t48192, t48195, t48201, t48204, t48216, t48218, t48219, t48220)
}
