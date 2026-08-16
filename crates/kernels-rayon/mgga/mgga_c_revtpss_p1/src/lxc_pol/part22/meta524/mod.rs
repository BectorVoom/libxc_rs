//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2301;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2302;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2303;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta524(t1188: f64, t17150: f64, t1749: f64, t3495: f64, t1161: f64, t1180: f64, t1189: f64, t12418: f64, t12476: f64, t17032: f64, t17086: f64, t17089: f64, t17094: f64, t17097: f64, t1745: f64, t1757: f64, t3447: f64, t3472: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3524: f64, t5120: f64, t5143: f64, t5158: f64, t5181: f64, t16954: f64, t16995: f64, t17029: f64, t300: f64, t3535: f64, t5192: f64, t1179: f64, t1196: f64, t3531: f64, t5207: f64, t16783: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t16781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17151, t17154, t17157) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2301(t1188, t17150, t1749, t3495, t1161, t1180, t1189, t12418, t12476, t17032, t17086, t17089, t17094, t17097, t1745, t1757, t3447, t3472, t3480, t3491, t3498, t3516, t3524, t5120, t5143, t5158, t5181);
        let (t17160, t17162, t17164, t17166, t17168) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2302(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2303(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let t17170 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2304(t16781, t17169);
    (t17151, t17154, t17160, t17162, t17164, t17166, t17168, t17170)
}
