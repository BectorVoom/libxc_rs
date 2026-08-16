//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2217;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2218;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2219;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta546<F: Float>(t16954: F, t16995: F, t17029: F, t17157: F, t300: F, t3535: F, t5192: F, t1179: F, t1188: F, t17150: F, t1196: F, t3531: F, t5207: F, t16783: F, t16786: F, t16788: F, t16790: F, t16809: F, t16814: F, t16834: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t17094: F, t16781: F, t1287: F, t487: F, t3584: F, t5486: F, t16756: F, t5480: F, t1770: F, t3781: F, t1234: F, t12709: F, t12756: F, t1285: F, t1291: F, t16697: F, t16751: F, t16757: F, t16763: F, t16768: F, t16772: F, t16776: F, t3666: F, t3670: F, t3746: F, t3760: F, t3763: F, t3784: F, t5216: F, t5326: F, t5459: F, t5463: F, t5474: F, t5478: F, t5487: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17160, t17162, t17164, t17166, t17168) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2217::<F>(t16954, t16995, t17029, t17157, t300, t3535, t5192, t1179, t1188, t17150, t1196, t3531, t5207);
        let t17169 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2218::<F>(t16783, t16786, t16788, t16790, t16809, t16814, t16834, t16837, t16839, t16842, t16844, t16846, t16945, t17094, t17160, t17162, t17166, t17168);
        let t17170 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2219::<F>(t16781, t17169);
        let (t17172, t17175, t17178, t17183, t17186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2220::<F>(t1287, t17170, t487, t3584, t5486, t16756, t5480, t1770, t3781, t1234, t12709, t12756, t1285, t1291, t16697, t16751, t16757, t16763, t16768, t16772, t16776, t3666, t3670, t3746, t3760, t3763, t3784, t5216, t5326, t5459, t5463, t5474, t5478, t5487);
    (t17160, t17162, t17164, t17166, t17168, t17170, t17172, t17175, t17178, t17183, t17186)
}
