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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta682(t14165: f64, t44505: f64, t11557: f64, t4889: f64, t11560: f64, t1174: f64, t1716: f64, t2402: f64, t4930: f64, t698: f64, t11513: f64, t11589: f64, t15313: f64, t3447: f64, t14749: f64, t15402: f64, t11499: f64, t11505: f64, t44424: f64, t44439: f64, t44504: f64, t52050: f64, t52053: f64, t52058: f64, t52061: f64, t52064: f64, t44620: f64, t461: f64, t60: f64, t15394: f64, t1714: f64, t3439: f64, t4724: f64, t697: f64, t11590: f64, t15376: f64, t11554: f64, t1706: f64, t44579: f64, t4904: f64, t11545: f64, t134: f64, t14726: f64, t11579: f64, t15338: f64, t4899: f64, t4928: f64, t11563: f64, t11571: f64, t11572: f64, t11575: f64, t15390: f64, t15395: f64, t44506: f64, t44521: f64, t44608: f64, t4908: f64, t50865: f64, t50869: f64, t50910: f64, t50924: f64, t11570: f64, t12648: f64, t10913: f64, t14730: f64, t1409: f64, t3450: f64, t3469: f64, t14725: f64, t15288: f64, t1090: f64, t11526: f64, t11569: f64, t11593: f64, t15293: f64, t24705: f64, t3449: f64, t44415: f64, t44419: f64, t44445: f64, t44478: f64, t44481: f64, t44487: f64, t4900: f64, t4919: f64, t50959: f64, t11583: f64, t12652: f64, t44607: f64, t4723: f64, t11536: f64, t15268: f64, t15281: f64, t1184: f64, t15320: f64, t15357: f64, t15382: f64, t24698: f64, t3243: f64, t3248: f64, t3252: f64, t44499: f64, t44502: f64, t44529: f64, t460: f64, t4934: f64, t7319: f64, t1709: f64, t44633: f64, t11530: f64, t15273: f64, t11533: f64, t11496: f64, t11502: f64, t11510: f64, t11518: f64, t11522: f64, t1177: f64, t1178: f64, t3475: f64, t44512: f64, t44527: f64, t44564: f64, t44573: f64, t44581: f64, t45872: f64, t50853: f64, t43768: f64, t43770: f64, t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t44466: f64, t50824: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64, t50881: f64, t50886: f64, t51039: f64, t51051: f64, t43859: f64, t43861: f64, t43863: f64, t50968: f64, t50970: f64, t50972: f64, t50976: f64, t50978: f64, t50987: f64, t50990: f64, t51034: f64, t51037: f64, t51041: f64, t51043: f64, t51046: f64, t51049: f64, t51053: f64, t51056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52066, t52074, t52076, t52081, t52085, t52086, t52089) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2574(t14165, t44505, t11557, t4889, t11560, t1174, t1716, t2402, t4930, t698, t11513, t11589, t15313, t3447);
        let t52094 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2575(t14749, t15402, t3447, t11499, t11505, t44424, t44439, t44504, t4889, t52050, t52053, t52058, t52061, t52064, t52066, t52074, t52076, t52081, t52085, t52086, t52089);
        let (t52096, t52100, t52110, t52122, t52124) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2576(t44620, t461, t60, t15394, t1714, t3439, t3447, t4724, t697, t11590, t15376, t11554, t1706);
        let t52150 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2577(t3447, t44579, t4904, t11545, t134, t461, t14726, t11579, t15338, t4899, t4928, t11563, t11571, t11572, t11575, t15313, t15376, t15390, t15395, t44506, t44521, t44608, t4908, t50865, t50869, t50910, t50924, t52096, t52100, t52110, t52122, t52124);
        let (t52165, t52183, t52197) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2578(t11570, t12648, t10913, t14730, t1409, t3450, t3469, t14725, t15288, t15338, t3447, t1090, t11526, t11569, t11575, t11593, t15293, t15390, t15395, t24705, t3449, t44415, t44419, t44445, t44478, t44481, t44487, t4889, t4900, t4919, t50959);
        let (t52216, t52220, t52224, t52228, t52236, t52240, t52250) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2579(t11583, t12652, t12648, t11570, t14165, t44607, t10913, t4723, t11536, t4889, t1174, t15268, t15281);
        let t52257 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2580(t1090, t11569, t1174, t1184, t15288, t15320, t15357, t15382, t15390, t24698, t3243, t3248, t3252, t3447, t3449, t3469, t44499, t44502, t44529, t460, t4908, t4919, t4928, t4934, t52216, t52220, t52224, t52228, t52236, t52240, t52250, t7319);
        let t52303 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2581(t11570, t12652, t1174, t1709, t44633, t11530, t4889, t15273, t15281, t11533, t11496, t11502, t11510, t11518, t11522, t11569, t1177, t1178, t1714, t3447, t3475, t44512, t44527, t44564, t44573, t44581, t45872, t460, t4928, t4934);
        let t52327 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582(t50853, t43768, t43770, t43835, t43837, t43839, t43855, t43857, t44466, t50824, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875, t50881, t50886);
        let t52345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2583(t51039, t51051, t43859, t43861, t43863, t50968, t50970, t50972, t50976, t50978, t50987, t50990, t51034, t51037, t51041, t51043, t51046, t51049, t51053, t51056);
    (t52094, t52150, t52165, t52183, t52197, t52236, t52257, t52303, t52327, t52345)
}
