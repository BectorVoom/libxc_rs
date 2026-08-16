//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2169;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2170;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2171;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2172;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2173;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta697<F: Float>(t5303: F, t91100: F, t1339: F, t550: F, t56812: F, t6936: F, t12289: F, t1351: F, t57342: F, t20473: F, t3788: F, t19930: F, t6952: F, t1831: F, t91191: F, t26257: F, t5314: F, t28100: F, t80853: F, t80855: F, t80767: F, t80776: F, t80780: F, t91206: F, t91215: F, t91226: F, t97310: F, t97315: F, t97318: F, t97320: F, t22788: F, t6431: F, t6427: F, t19996: F, t5310: F, t1358: F, t28088: F, t22852: F, t3792: F, t80798: F, t97312: F, t22705: F, t236: F, t6414: F, t80784: F, t80792: F, t80794: F, t80826: F, t80837: F, t80848: F, t91282: F, t91284: F, t91287: F, t91290: F, t91301: F, t22765: F, t6417: F, t6390: F, t80997: F, t81000: F, t6388: F, t19958: F, t22833: F, t80867: F, t80886: F, t91304: F, t91311: F, t91323: F, t91328: F, t91345: F, t91346: F, t91357: F, t91359: F, t91365: F, t93721: F, t93723: F, t22797: F, t6375: F, t19732: F, t22779: F, t28057: F, t6371: F, t80827: F, t28073: F, t80888: F, t26301: F, t7708: F, t91208: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t97322, t97326, t97333, t97337, t97340) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2169::<F>(t5303, t91100, t1339, t550, t56812, t6936, t12289, t1351, t57342, t20473, t3788, t19930, t6952);
        let t97349 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2170::<F>(t1831, t91191, t26257, t5314, t28100, t80853, t80855, t80767, t80776, t80780, t91206, t91215, t91226, t97310, t97315, t97318, t97320, t97322, t97326, t97333, t97337, t97340);
        let (t97352, t97354, t97359, t97361, t97363, t97367) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2171::<F>(t22788, t6431, t6427, t19996, t6952, t26257, t5310, t1358, t28088, t22852, t3792, t80798, t97312);
        let t97376 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2172::<F>(t22705, t22852, t236, t550, t6414, t80784, t80792, t80794, t80826, t80837, t80848, t91282, t91284, t91287, t91290, t91301, t97352, t97354, t97359, t97361, t97363, t97367);
        let t97392 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2173::<F>(t22765, t6417, t6390, t80997, t81000, t1351, t3788, t6388, t6936, t19958, t22833, t80867, t80886, t91304, t91311, t91323, t91328, t91345, t91346, t91357, t91359, t91365, t93721, t93723);
        let (t97394, t97398, t97400, t97402, t97404, t97407) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2174::<F>(t22797, t6375, t1339, t19732, t6936, t22779, t28057, t6371, t80827, t28073, t80888, t26301, t7708, t91208);
    (t97349, t97376, t97392, t97394, t97398, t97400, t97402, t97404, t97407)
}
