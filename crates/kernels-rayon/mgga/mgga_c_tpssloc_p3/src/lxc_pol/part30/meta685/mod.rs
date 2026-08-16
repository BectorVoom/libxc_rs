//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2161;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2162;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2163;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2164;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2165;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta685(t5303: f64, t91100: f64, t1339: f64, t550: f64, t56812: f64, t6936: f64, t12289: f64, t1351: f64, t57342: f64, t20473: f64, t3788: f64, t19930: f64, t6952: f64, t1831: f64, t91191: f64, t26257: f64, t5314: f64, t28100: f64, t80853: f64, t80855: f64, t80767: f64, t80776: f64, t80780: f64, t91206: f64, t91215: f64, t91226: f64, t97310: f64, t97315: f64, t97318: f64, t97320: f64, t22788: f64, t6431: f64, t6427: f64, t19996: f64, t5310: f64, t1358: f64, t28088: f64, t22852: f64, t3792: f64, t80798: f64, t97312: f64, t22705: f64, t236: f64, t6414: f64, t80784: f64, t80792: f64, t80794: f64, t80826: f64, t80837: f64, t80848: f64, t91282: f64, t91284: f64, t91287: f64, t91290: f64, t91301: f64, t22765: f64, t6417: f64, t6390: f64, t80997: f64, t81000: f64, t6388: f64, t19958: f64, t22833: f64, t80867: f64, t80886: f64, t91304: f64, t91311: f64, t91323: f64, t91328: f64, t91345: f64, t91346: f64, t91357: f64, t91359: f64, t91365: f64, t93721: f64, t93723: f64, t22797: f64, t6375: f64, t19732: f64, t22779: f64, t28057: f64, t6371: f64, t80827: f64, t28073: f64, t80888: f64, t26301: f64, t7708: f64, t91208: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97322, t97326, t97333, t97337, t97340) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2161(t5303, t91100, t1339, t550, t56812, t6936, t12289, t1351, t57342, t20473, t3788, t19930, t6952);
        let t97349 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2162(t1831, t91191, t26257, t5314, t28100, t80853, t80855, t80767, t80776, t80780, t91206, t91215, t91226, t97310, t97315, t97318, t97320, t97322, t97326, t97333, t97337, t97340);
        let (t97352, t97354, t97359, t97361, t97363, t97367) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2163(t22788, t6431, t6427, t19996, t6952, t26257, t5310, t1358, t28088, t22852, t3792, t80798, t97312);
        let t97376 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2164(t22705, t22852, t236, t550, t6414, t80784, t80792, t80794, t80826, t80837, t80848, t91282, t91284, t91287, t91290, t91301, t97352, t97354, t97359, t97361, t97363, t97367);
        let t97392 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2165(t22765, t6417, t6390, t80997, t81000, t1351, t3788, t6388, t6936, t19958, t22833, t80867, t80886, t91304, t91311, t91323, t91328, t91345, t91346, t91357, t91359, t91365, t93721, t93723);
        let (t97394, t97398, t97400, t97402, t97404, t97407) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2166(t22797, t6375, t1339, t19732, t6936, t22779, t28057, t6371, t80827, t28073, t80888, t26301, t7708, t91208);
    (t97349, t97376, t97392, t97394, t97398, t97400, t97402, t97404, t97407)
}
