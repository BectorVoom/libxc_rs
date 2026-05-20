//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2301;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2302;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2303;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta524<F: Float>(t1188: F, t17150: F, t1749: F, t3495: F, t1161: F, t1180: F, t1189: F, t12418: F, t12476: F, t17032: F, t17086: F, t17089: F, t17094: F, t17097: F, t1745: F, t1757: F, t3447: F, t3472: F, t3480: F, t3491: F, t3498: F, t3516: F, t3524: F, t5120: F, t5143: F, t5158: F, t5181: F, t16954: F, t16995: F, t17029: F, t300: F, t3535: F, t5192: F, t1179: F, t1196: F, t3531: F, t5207: F, t16783: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t16781: F) -> (F, F, F, F, F, F, F, F) {
        let (t17151, t17154, t17157) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2301::<F>(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
        let (t17160, t17162, t17164, t17166, t17168) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2302::<F>(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2303::<F>(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let t17170 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2304::<F>(t16781, t17169);
    (t17151, t17154, t17160, t17162, t17164, t17166, t17168, t17170)
}
