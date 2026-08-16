//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta785 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta785(t57229: f64, t40227: f64, t40231: f64, t40233: f64, t118: f64, t2375: f64, t6320: f64, t54477: f64, t40224: f64, t40230: f64, t57218: f64, t57219: f64, t57220: f64, t57221: f64, t57222: f64, t57223: f64, t57224: f64, t57225: f64, t57226: f64, t57228: f64, t1307: f64, t1365: f64, t16018: f64, t16186: f64, t16192: f64, t16196: f64, t16199: f64, t19631: f64, t19708: f64, t19715: f64, t19716: f64, t19719: f64, t19724: f64, t225: f64, t3719: f64, t3734: f64, t3844: f64, t5272: f64, t5278: f64, t5279: f64, t5280: f64, t548: f64, t57193: f64, t57194: f64, t57196: f64, t57197: f64, t57200: f64, t57201: f64, t57217: f64, t6330: f64, t6404: f64, t68: f64, t6924: f64, t1345: f64, t1347: f64, t1348: f64, t16148: f64, t16176: f64, t16191: f64, t16202: f64, t1819: f64, t1821: f64, t19702: f64, t19725: f64, t19728: f64, t1995: f64, t3839: f64, t3843: f64, t3847: f64, t5283: f64, t546: f64, t56275: f64, t56486: f64, t6347: f64, t6408: f64, t6411: f64, t550: f64, t12215: f64, t12397: f64, t12419: f64, t1341: f64, t1343: f64, t16206: f64, t1810: f64, t19868: f64, t19871: f64, t19979: f64, t210: f64, t3733: f64, t3778: f64, t3803: f64, t3807: f64, t3856: f64, t39952: f64, t39975: f64, t40160: f64, t5246: f64, t5248: f64, t5249: f64, t54063: f64, t57143: f64, t57145: f64, t57147: f64, t57158: f64, t57160: f64, t57170: f64, t57172: f64, t6370: f64, t6390: f64, t6396: f64, t6417: f64, t820: f64, t12300: f64, t6422: f64, t12365: f64, t1358: f64, t19836: f64, t12250: f64, t6387: f64, t12429: f64, t16101: f64, t16215: f64, t16217: f64, t16225: f64, t16233: f64, t16305: f64, t16311: f64, t16312: f64, t16401: f64, t1825: f64, t19735: f64, t19886: f64, t19890: f64, t221: f64, t5240: f64, t53973: f64, t54555: f64, t54557: f64, t54561: f64, t54567: f64, t56560: f64, t57086: f64, t6388: f64, t6394: f64, t56913: f64, t3862: f64, t6379: f64, t5293: f64, t53945: f64, t19921: f64, t3866: f64, t19926: f64, t1352: f64, t1363: f64, t16394: f64, t16405: f64, t19843: f64, t19972: f64, t19996: f64, t20000: f64, t3783: f64, t3870: f64, t40025: f64, t40282: f64, t53990: f64, t54162: f64, t54582: f64, t56817: f64, t6374: f64, t3799: f64, t12283: f64, t19958: f64, t12351: f64, t12407: f64, t16060: f64, t16153: f64, t16224: f64, t16391: f64, t1799: f64, t19876: f64, t19882: f64, t19956: f64, t3805: f64, t40293: f64, t5245: f64, t5252: f64, t54585: f64, t54607: f64, t54609: f64, t54611: f64, t54750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57230, t57231, t57232, t57233, t57236, t57237, t57238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2719(t57229, t40227, t40231, t40233, t118, t2375, t6320, t54477, t40224, t40230, t57218, t57219, t57220, t57221, t57222, t57223, t57224, t57225, t57226, t57228);
        let t57266 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2720(t1307, t1365, t16018, t16186, t16192, t16196, t16199, t19631, t19708, t19715, t19716, t19719, t19724, t225, t3719, t3734, t3844, t5272, t5278, t5279, t5280, t548, t57193, t57194, t57196, t57197, t57200, t57201, t57217, t57238, t6330, t6404, t68, t6924);
        let t57298 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2721(t1345, t1347, t1348, t16148, t16176, t16186, t16191, t16202, t1819, t1821, t19702, t19725, t19728, t1995, t3734, t3839, t3843, t3847, t5272, t5278, t5283, t546, t56275, t56486, t6347, t6404, t6408, t6411);
        let (t57300, t57305) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2722(t550, t57266, t57298, t12215, t12397, t12419, t1341, t1343, t16018, t16206, t1810, t19868, t19871, t19979, t210, t3719, t3733, t3778, t3803, t3807, t3856, t39952, t39975, t40160, t5246, t5248, t5249, t54063, t57143, t57145, t57147, t57158, t57160, t57170, t57172, t6370, t6390, t6396, t6417, t820);
        let t57351 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723(t12300, t6422, t12365, t1358, t19836, t12250, t6387, t12429, t16101, t16215, t16217, t16225, t16233, t16305, t16311, t16312, t16401, t1825, t19735, t19886, t19890, t221, t3803, t5240, t5246, t53973, t54063, t54555, t54557, t54561, t54567, t56560, t57086, t6388, t6394);
        let (t57354, t57400) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2724(t550, t56913, t3862, t6379, t5293, t53945, t19921, t3866, t19926, t12215, t12397, t12429, t1307, t1341, t1343, t1352, t1363, t16394, t16405, t19631, t19843, t19972, t19996, t20000, t210, t3733, t3734, t3783, t3803, t3870, t40025, t40282, t5248, t53990, t54162, t54582, t56817, t6370, t6374, t6422, t820);
        let t57447 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725(t12300, t6417, t19868, t3799, t12283, t19958, t12351, t12407, t12429, t1363, t16018, t16060, t16148, t16153, t16224, t16391, t1799, t1825, t19876, t19882, t19956, t3719, t3803, t3805, t3807, t3870, t40293, t5245, t5252, t54585, t54607, t54609, t54611, t54750, t56817, t6330, t820);
    (t57230, t57231, t57232, t57233, t57236, t57237, t57300, t57305, t57351, t57354, t57400, t57447)
}
