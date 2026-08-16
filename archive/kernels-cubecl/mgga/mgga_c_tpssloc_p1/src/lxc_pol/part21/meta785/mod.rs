//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta785 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta785<F: Float>(t57229: F, t40227: F, t40231: F, t40233: F, t118: F, t2375: F, t6320: F, t54477: F, t40224: F, t40230: F, t57218: F, t57219: F, t57220: F, t57221: F, t57222: F, t57223: F, t57224: F, t57225: F, t57226: F, t57228: F, t1307: F, t1365: F, t16018: F, t16186: F, t16192: F, t16196: F, t16199: F, t19631: F, t19708: F, t19715: F, t19716: F, t19719: F, t19724: F, t225: F, t3719: F, t3734: F, t3844: F, t5272: F, t5278: F, t5279: F, t5280: F, t548: F, t57193: F, t57194: F, t57196: F, t57197: F, t57200: F, t57201: F, t57217: F, t6330: F, t6404: F, t68: F, t6924: F, t1345: F, t1347: F, t1348: F, t16148: F, t16176: F, t16191: F, t16202: F, t1819: F, t1821: F, t19702: F, t19725: F, t19728: F, t1995: F, t3839: F, t3843: F, t3847: F, t5283: F, t546: F, t56275: F, t56486: F, t6347: F, t6408: F, t6411: F, t550: F, t12215: F, t12397: F, t12419: F, t1341: F, t1343: F, t16206: F, t1810: F, t19868: F, t19871: F, t19979: F, t210: F, t3733: F, t3778: F, t3803: F, t3807: F, t3856: F, t39952: F, t39975: F, t40160: F, t5246: F, t5248: F, t5249: F, t54063: F, t57143: F, t57145: F, t57147: F, t57158: F, t57160: F, t57170: F, t57172: F, t6370: F, t6390: F, t6396: F, t6417: F, t820: F, t12300: F, t6422: F, t12365: F, t1358: F, t19836: F, t12250: F, t6387: F, t12429: F, t16101: F, t16215: F, t16217: F, t16225: F, t16233: F, t16305: F, t16311: F, t16312: F, t16401: F, t1825: F, t19735: F, t19886: F, t19890: F, t221: F, t5240: F, t53973: F, t54555: F, t54557: F, t54561: F, t54567: F, t56560: F, t57086: F, t6388: F, t6394: F, t56913: F, t3862: F, t6379: F, t5293: F, t53945: F, t19921: F, t3866: F, t19926: F, t1352: F, t1363: F, t16394: F, t16405: F, t19843: F, t19972: F, t19996: F, t20000: F, t3783: F, t3870: F, t40025: F, t40282: F, t53990: F, t54162: F, t54582: F, t56817: F, t6374: F, t3799: F, t12283: F, t19958: F, t12351: F, t12407: F, t16060: F, t16153: F, t16224: F, t16391: F, t1799: F, t19876: F, t19882: F, t19956: F, t3805: F, t40293: F, t5245: F, t5252: F, t54585: F, t54607: F, t54609: F, t54611: F, t54750: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57230, t57231, t57232, t57233, t57236, t57237, t57238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719::<F>(t57229, t40227, t40231, t40233, t118, t2375, t6320, t54477, t40224, t40230, t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228);
        let t57266 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720::<F>(t1307, t1365, t16018, t16186, t16192, t16196, t16199, t19631, t19708, t19715, t19716, t19719, t19724, t225, t3719, t3734, t3844, t5272, t5278, t5279, t5280, t548, t57193, t57194, t57196, t57197, t57200, t57201, t57217, t57238, t6330, t6404, t68, t6924);
        let t57298 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721::<F>(t1345, t1347, t1348, t16148, t16176, t16186, t16191, t16202, t1819, t1821, t19702, t19725, t19728, t1995, t3734, t3839, t3843, t3847, t5272, t5278, t5283, t546, t56275, t56486, t6347, t6404, t6408, t6411);
        let (t57300, t57305) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722::<F>(t550, t57266, t57298, t12215, t12397, t12419, t1341, t1343, t16018, t16206, t1810, t19868, t19871, t19979, t210, t3719, t3733, t3778, t3803, t3807, t3856, t39952, t39975, t40160, t5246, t5248, t5249, t54063, t57143, t57145, t57147, t57158, t57160, t57170, t57172, t6370, t6390, t6396, t6417, t820);
        let t57351 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723::<F>(t12300, t6422, t12365, t1358, t19836, t12250, t6387, t12429, t16101, t16215, t16217, t16225, t16233, t16305, t16311, t16312, t16401, t1825, t19735, t19886, t19890, t221, t3803, t5240, t5246, t53973, t54063, t54555, t54557, t54561, t54567, t56560, t57086, t6388, t6394);
        let (t57354, t57400) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724::<F>(t550, t56913, t3862, t6379, t5293, t53945, t19921, t3866, t19926, t12215, t12397, t12429, t1307, t1341, t1343, t1352, t1363, t16394, t16405, t19631, t19843, t19972, t19996, t20000, t210, t3733, t3734, t3783, t3803, t3870, t40025, t40282, t5248, t53990, t54162, t54582, t56817, t6370, t6374, t6422, t820);
        let t57447 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725::<F>(t12300, t6417, t19868, t3799, t12283, t19958, t12351, t12407, t12429, t1363, t16018, t16060, t16148, t16153, t16224, t16391, t1799, t1825, t19876, t19882, t19956, t3719, t3803, t3805, t3807, t3870, t40293, t5245, t5252, t54585, t54607, t54609, t54611, t54750, t56817, t6330, t820);
    (t57230, t57231, t57232, t57233, t57236, t57237, t57300, t57305, t57351, t57354, t57400, t57447)
}
