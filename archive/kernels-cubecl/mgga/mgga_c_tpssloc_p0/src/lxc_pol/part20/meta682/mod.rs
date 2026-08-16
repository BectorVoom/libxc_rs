//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta682 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2574;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2575;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2576;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2577;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2578;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2579;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2580;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2581;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta682<F: Float>(t14165: F, t44505: F, t11557: F, t4889: F, t11560: F, t1174: F, t1716: F, t2402: F, t4930: F, t698: F, t11513: F, t11589: F, t15313: F, t3447: F, t14749: F, t15402: F, t11499: F, t11505: F, t44424: F, t44439: F, t44504: F, t52050: F, t52053: F, t52058: F, t52061: F, t52064: F, t44620: F, t461: F, t60: F, t15394: F, t1714: F, t3439: F, t4724: F, t697: F, t11590: F, t15376: F, t11554: F, t1706: F, t44579: F, t4904: F, t11545: F, t134: F, t14726: F, t11579: F, t15338: F, t4899: F, t4928: F, t11563: F, t11571: F, t11572: F, t11575: F, t15390: F, t15395: F, t44506: F, t44521: F, t44608: F, t4908: F, t50865: F, t50869: F, t50910: F, t50924: F, t11570: F, t12648: F, t10913: F, t14730: F, t1409: F, t3450: F, t3469: F, t14725: F, t15288: F, t1090: F, t11526: F, t11569: F, t11593: F, t15293: F, t24705: F, t3449: F, t44415: F, t44419: F, t44445: F, t44478: F, t44481: F, t44487: F, t4900: F, t4919: F, t50959: F, t11583: F, t12652: F, t44607: F, t4723: F, t11536: F, t15268: F, t15281: F, t1184: F, t15320: F, t15357: F, t15382: F, t24698: F, t3243: F, t3248: F, t3252: F, t44499: F, t44502: F, t44529: F, t460: F, t4934: F, t7319: F, t1709: F, t44633: F, t11530: F, t15273: F, t11533: F, t11496: F, t11502: F, t11510: F, t11518: F, t11522: F, t1177: F, t1178: F, t3475: F, t44512: F, t44527: F, t44564: F, t44573: F, t44581: F, t45872: F, t50853: F, t43768: F, t43770: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t44466: F, t50824: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F, t50881: F, t50886: F, t51039: F, t51051: F, t43859: F, t43861: F, t43863: F, t50968: F, t50970: F, t50972: F, t50976: F, t50978: F, t50987: F, t50990: F, t51034: F, t51037: F, t51041: F, t51043: F, t51046: F, t51049: F, t51053: F, t51056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t52066, t52074, t52076, t52081, t52085, t52086, t52089) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2574::<F>(t14165, t44505, t11557, t4889, t11560, t1174, t1716, t2402, t4930, t698, t11513, t11589, t15313, t3447);
        let t52094 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2575::<F>(t14749, t15402, t3447, t11499, t11505, t44424, t44439, t44504, t4889, t52050, t52053, t52058, t52061, t52064, t52066, t52074, t52076, t52081, t52085, t52086, t52089);
        let (t52096, t52100, t52110, t52122, t52124) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2576::<F>(t44620, t461, t60, t15394, t1714, t3439, t3447, t4724, t697, t11590, t15376, t11554, t1706);
        let t52150 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2577::<F>(t3447, t44579, t4904, t11545, t134, t461, t14726, t11579, t15338, t4899, t4928, t11563, t11571, t11572, t11575, t15313, t15376, t15390, t15395, t44506, t44521, t44608, t4908, t50865, t50869, t50910, t50924, t52096, t52100, t52110, t52122, t52124);
        let (t52165, t52183, t52197) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2578::<F>(t11570, t12648, t10913, t14730, t1409, t3450, t3469, t14725, t15288, t15338, t3447, t1090, t11526, t11569, t11575, t11593, t15293, t15390, t15395, t24705, t3449, t44415, t44419, t44445, t44478, t44481, t44487, t4889, t4900, t4919, t50959);
        let (t52216, t52220, t52224, t52228, t52236, t52240, t52250) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2579::<F>(t11583, t12652, t12648, t11570, t14165, t44607, t10913, t4723, t11536, t4889, t1174, t15268, t15281);
        let t52257 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2580::<F>(t1090, t11569, t1174, t1184, t15288, t15320, t15357, t15382, t15390, t24698, t3243, t3248, t3252, t3447, t3449, t3469, t44499, t44502, t44529, t460, t4908, t4919, t4928, t4934, t52216, t52220, t52224, t52228, t52236, t52240, t52250, t7319);
        let t52303 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2581::<F>(t11570, t12652, t1174, t1709, t44633, t11530, t4889, t15273, t15281, t11533, t11496, t11502, t11510, t11518, t11522, t11569, t1177, t1178, t1714, t3447, t3475, t44512, t44527, t44564, t44573, t44581, t45872, t460, t4928, t4934);
        let t52327 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582::<F>(t50853, t43768, t43770, t43835, t43837, t43839, t43855, t43857, t44466, t50824, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875, t50881, t50886);
        let t52345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583::<F>(t51039, t51051, t43859, t43861, t43863, t50968, t50970, t50972, t50976, t50978, t50987, t50990, t51034, t51037, t51041, t51043, t51046, t51049, t51053, t51056);
    (t52094, t52150, t52165, t52183, t52197, t52236, t52257, t52303, t52327, t52345)
}
