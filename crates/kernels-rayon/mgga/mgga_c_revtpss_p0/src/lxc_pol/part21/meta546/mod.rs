//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2217;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2218;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2219;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta546(t16954: f64, t16995: f64, t17029: f64, t17157: f64, t300: f64, t3535: f64, t5192: f64, t1179: f64, t1188: f64, t17150: f64, t1196: f64, t3531: f64, t5207: f64, t16783: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t17094: f64, t16781: f64, t1287: f64, t487: f64, t3584: f64, t5486: f64, t16756: f64, t5480: f64, t1770: f64, t3781: f64, t1234: f64, t12709: f64, t12756: f64, t1285: f64, t1291: f64, t16697: f64, t16751: f64, t16757: f64, t16763: f64, t16768: f64, t16772: f64, t16776: f64, t3666: f64, t3670: f64, t3746: f64, t3760: f64, t3763: f64, t3784: f64, t5216: f64, t5326: f64, t5459: f64, t5463: f64, t5474: f64, t5478: f64, t5487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17160, t17162, t17164, t17166, t17168) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2217(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2218(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let t17170 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2219(t16781, t17169);
        let (t17172, t17175, t17178, t17183, t17186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2220(t1287, t17170, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
    (t17160, t17162, t17164, t17166, t17168, t17170, t17172, t17175, t17178, t17183, t17186)
}
