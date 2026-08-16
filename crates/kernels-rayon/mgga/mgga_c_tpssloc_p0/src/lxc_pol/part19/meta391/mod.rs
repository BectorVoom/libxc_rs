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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta391(t1213: f64, t1216: f64, t248: f64, t45017: f64, t11862: f64, t1227: f64, t13969: f64, t11716: f64, t44833: f64, t44834: f64, t3503: f64, t1174: f64, t1197: f64, t2402: f64, t3584: f64, t676: f64, t3243: f64, t11159: f64, t11665: f64, t11668: f64, t11678: f64, t11684: f64, t11721: f64, t1177: f64, t11805: f64, t1214: f64, t15620: f64, t15661: f64, t15708: f64, t2250: f64, t3247: f64, t3490: f64, t3508: f64, t3577: f64, t3578: f64, t42374: f64, t43723: f64, t44699: f64, t45002: f64, t45007: f64, t45009: f64, t45013: f64, t45015: f64, t4582: f64, t4987: f64, t1011: f64, t1212: f64, t44706: f64, t11692: f64, t11693: f64, t11697: f64, t11853: f64, t3570: f64, t11163: f64, t3521: f64, t221: f64, t44483: f64, t456: f64, t3575: f64, t42386: f64, t11888: f64, t11914: f64, t11784: f64, t820: f64, t11669: f64, t11779: f64, t1090: f64, t11148: f64, t11172: f64, t11670: f64, t11729: f64, t11739: f64, t11809: f64, t11825: f64, t1218: f64, t1230: f64, t3531: f64, t43800: f64, t43804: f64, t11677: f64, t11907: f64, t11769: f64, t3515: f64, t11904: f64, t11702: f64, t3536: f64, t11709: f64, t11745: f64, t11651: f64, t11734: f64, t3556: f64, t698: f64, t11844: f64, t135: f64, t11849: f64, t11662: f64, t11680: f64, t11688: f64, t11694: f64, t3248: f64, t3252: f64, t3506: f64, t3509: f64, t3516: f64, t3560: f64, t39103: f64, t44774: f64, t44879: f64, t484: f64, t488: f64, t4978: f64, t68: f64, t974: f64, t11153: f64, t1176: f64, t11881: f64, t11773: f64, t11168: f64, t3431: f64, t3540: f64, t3567: f64, t11539: f64, t11154: f64, t11546: f64, t11722: f64, t11855: f64, t11863: f64, t1196: f64, t3440: f64, t3494: f64, t39097: f64, t39110: f64, t43711: f64, t43715: f64, t43732: f64, t4972: f64, t374: f64, t485: f64, t486: f64, t9697: f64, t3493: f64, t11786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45020, t45027, t45030, t45037, t45044) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1471(t1213, t1216, t248, t45017, t11862, t1227, t13969, t11716, t44833, t44834, t3503, t1174, t1197, t2402);
        let t45066 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1472(t3584, t676, t1227, t248, t3243, t11159, t11665, t11668, t11678, t11684, t11721, t1174, t1177, t11805, t1214, t1216, t15620, t15661, t15708, t2250, t3247, t3490, t3508, t3577, t3578, t42374, t43723, t44699, t45002, t45007, t45009, t45013, t45015, t45020, t45027, t45030, t45037, t45044, t4582, t4987);
        let (t45080, t45086, t45102, t45108) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1473(t1011, t1212, t44706, t11692, t11693, t11697, t11853, t1213, t248, t3570, t11163, t1227, t3521);
        let (t45112, t45113, t45114, t45119, t45126, t45128) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1474(t221, t44483, t456, t3575, t42386, t11888, t11914, t11784, t820, t11669, t3577, t11779);
        let t45133 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1475(t1090, t11148, t11163, t11172, t11665, t11670, t11729, t11739, t11809, t11825, t11853, t1216, t1218, t1227, t1230, t248, t3490, t3531, t3577, t3578, t43800, t43804, t45080, t45086, t45102, t45108, t45112, t45114, t45119, t45126, t45128);
        let (t45134, t45148, t45162, t45167, t45169, t45171) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1476(t11677, t11907, t11769, t13969, t3515, t11904, t11702, t3536, t11709, t11745, t11651, t11734);
        let t45186 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1477(t1174, t3556, t698, t11844, t135, t11849, t11662, t11665, t11678, t11680, t11688, t11692, t11694, t11709, t3248, t3252, t3506, t3509, t3516, t3560, t3578, t39103, t44774, t44879, t45134, t45148, t45162, t45167, t45169, t45171, t4582, t484, t488, t4978, t68, t974);
        let (t45192, t45197, t45211, t45222, t45224) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1478(t11153, t1176, t11881, t45113, t11773, t1227, t13969, t11168, t1174, t3431, t3540, t3567);
        let t45246 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1479(t11159, t11539, t1174, t1090, t11154, t11546, t11668, t11678, t11722, t11855, t11863, t1196, t1216, t1227, t3243, t3248, t3252, t3440, t3490, t3494, t3509, t3536, t3577, t3578, t39097, t39110, t42374, t43711, t43715, t43732, t45192, t45197, t45211, t45222, t45224, t4582, t4972, t974);
        let (t45250, t45251, t45256, t45260) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1480(t374, t485, t486, t9697, t1090, t3493, t11786, t3490, t11154, t11784, t1227, t248);
    (t45066, t45133, t45186, t45246, t45250, t45251, t45256, t45260)
}
