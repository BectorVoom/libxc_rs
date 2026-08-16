//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1471;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1473;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1474;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1475;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1476;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1477;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1478;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1479;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta391<F: Float>(t1213: F, t1216: F, t248: F, t45017: F, t11862: F, t1227: F, t13969: F, t11716: F, t44833: F, t44834: F, t3503: F, t1174: F, t1197: F, t2402: F, t3584: F, t676: F, t3243: F, t11159: F, t11665: F, t11668: F, t11678: F, t11684: F, t11721: F, t1177: F, t11805: F, t1214: F, t15620: F, t15661: F, t15708: F, t2250: F, t3247: F, t3490: F, t3508: F, t3577: F, t3578: F, t42374: F, t43723: F, t44699: F, t45002: F, t45007: F, t45009: F, t45013: F, t45015: F, t4582: F, t4987: F, t1011: F, t1212: F, t44706: F, t11692: F, t11693: F, t11697: F, t11853: F, t3570: F, t11163: F, t3521: F, t221: F, t44483: F, t456: F, t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11669: F, t11779: F, t1090: F, t11148: F, t11172: F, t11670: F, t11729: F, t11739: F, t11809: F, t11825: F, t1218: F, t1230: F, t3531: F, t43800: F, t43804: F, t11677: F, t11907: F, t11769: F, t3515: F, t11904: F, t11702: F, t3536: F, t11709: F, t11745: F, t11651: F, t11734: F, t3556: F, t698: F, t11844: F, t135: F, t11849: F, t11662: F, t11680: F, t11688: F, t11694: F, t3248: F, t3252: F, t3506: F, t3509: F, t3516: F, t3560: F, t39103: F, t44774: F, t44879: F, t484: F, t488: F, t4978: F, t68: F, t974: F, t11153: F, t1176: F, t11881: F, t11773: F, t11168: F, t3431: F, t3540: F, t3567: F, t11539: F, t11154: F, t11546: F, t11722: F, t11855: F, t11863: F, t1196: F, t3440: F, t3494: F, t39097: F, t39110: F, t43711: F, t43715: F, t43732: F, t4972: F, t374: F, t485: F, t486: F, t9697: F, t3493: F, t11786: F) -> (F, F, F, F, F, F, F, F) {
        let (t45020, t45027, t45030, t45037, t45044) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1471::<F>(t1213, t1216, t248, t45017, t11862, t1227, t13969, t11716, t44833, t44834, t3503, t1174, t1197, t2402);
        let t45066 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472::<F>(t3584, t676, t1227, t248, t3243, t11159, t11665, t11668, t11678, t11684, t11721, t1174, t1177, t11805, t1214, t1216, t15620, t15661, t15708, t2250, t3247, t3490, t3508, t3577, t3578, t42374, t43723, t44699, t45002, t45007, t45009, t45013, t45015, t45020, t45027, t45030, t45037, t45044, t4582, t4987);
        let (t45080, t45086, t45102, t45108) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1473::<F>(t1011, t1212, t44706, t11692, t11693, t11697, t11853, t1213, t248, t3570, t11163, t1227, t3521);
        let (t45112, t45113, t45114, t45119, t45126, t45128) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1474::<F>(t221, t44483, t456, t3575, t42386, t11888, t11914, t11784, t820, t11669, t3577, t11779);
        let t45133 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1475::<F>(t1090, t11148, t11163, t11172, t11665, t11670, t11729, t11739, t11809, t11825, t11853, t1216, t1218, t1227, t1230, t248, t3490, t3531, t3577, t3578, t43800, t43804, t45080, t45086, t45102, t45108, t45112, t45114, t45119, t45126, t45128);
        let (t45134, t45148, t45162, t45167, t45169, t45171) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1476::<F>(t11677, t11907, t11769, t13969, t3515, t11904, t11702, t3536, t11709, t11745, t11651, t11734);
        let t45186 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1477::<F>(t1174, t3556, t698, t11844, t135, t11849, t11662, t11665, t11678, t11680, t11688, t11692, t11694, t11709, t3248, t3252, t3506, t3509, t3516, t3560, t3578, t39103, t44774, t44879, t45134, t45148, t45162, t45167, t45169, t45171, t4582, t484, t488, t4978, t68, t974);
        let (t45192, t45197, t45211, t45222, t45224) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1478::<F>(t11153, t1176, t11881, t45113, t11773, t1227, t13969, t11168, t1174, t3431, t3540, t3567);
        let t45246 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1479::<F>(t11159, t11539, t1174, t1090, t11154, t11546, t11668, t11678, t11722, t11855, t11863, t1196, t1216, t1227, t3243, t3248, t3252, t3440, t3490, t3494, t3509, t3536, t3577, t3578, t39097, t39110, t42374, t43711, t43715, t43732, t45192, t45197, t45211, t45222, t45224, t4582, t4972, t974);
        let (t45250, t45251, t45256, t45260) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1480::<F>(t374, t485, t486, t9697, t1090, t3493, t11786, t3490, t11154, t11784, t1227, t248);
    (t45066, t45133, t45186, t45246, t45250, t45251, t45256, t45260)
}
