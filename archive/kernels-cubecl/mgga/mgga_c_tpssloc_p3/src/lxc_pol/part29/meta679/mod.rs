//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta679 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2276;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2277;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2278;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2279;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2280;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2281;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2282;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2283;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2284;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta679<F: Float>(t11153: F, t497: F, t27654: F, t491: F, t1235: F, t8034: F, t27434: F, t85639: F, t27821: F, t24600: F, t7301: F, t27798: F, t4935: F, t24615: F, t1090: F, t12648: F, t14165: F, t2128: F, t24589: F, t24590: F, t24601: F, t24603: F, t27411: F, t27433: F, t27549: F, t27774: F, t4728: F, t5059: F, t7287: F, t85661: F, t85669: F, t86403: F, t24637: F, t8009: F, t24588: F, t8020: F, t1184: F, t4929: F, t1715: F, t3469: F, t24645: F, t7999: F, t1186: F, t15789: F, t1716: F, t1761: F, t24567: F, t24571: F, t24605: F, t24611: F, t27406: F, t27437: F, t27445: F, t27453: F, t27799: F, t460: F, t7283: F, t7286: F, t7300: F, t86475: F, t2121: F, t3427: F, t8010: F, t24574: F, t27416: F, t27794: F, t27441: F, t27446: F, t1751: F, t225: F, t461: F, t11925: F, t14972: F, t24563: F, t24604: F, t24884: F, t27382: F, t27751: F, t3471: F, t7356: F, t8002: F, t8061: F, t85701: F, t85728: F, t86415: F, t27812: F, t8006: F, t85660: F, t23383: F, t7303: F, t7291: F, t11605: F, t1251: F, t2155: F, t24602: F, t27761: F, t27766: F, t27784: F, t3487: F, t3966: F, t51937: F, t7391: F, t85711: F, t85717: F, t85724: F, t85733: F, t2122: F, t94319: F, t8003: F, t11928: F, t15786: F, t24582: F, t24633: F, t27388: F, t27396: F, t27830: F, t3600: F, t5055: F, t8014: F, t85707: F, t85739: F, t85741: F, t85766: F, t27412: F, t5052: F, t7299: F, t15359: F, t15790: F, t2123: F, t24596: F, t24617: F, t27381: F, t27820: F, t3243: F, t3593: F, t4930: F, t7295: F, t7302: F, t7351: F, t7392: F, t85787: F, t85789: F, t86452: F, t7294: F, t3475: F, t1238: F, t15802: F, t1760: F, t24597: F, t24616: F, t24867: F, t24897: F, t27775: F, t3477: F, t3598: F, t4723: F, t4945: F, t52386: F, t8088: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94349, t94354, t94358, t94363, t94365, t94369, t94374) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2276::<F>(t11153, t497, t27654, t491, t1235, t8034, t27434, t85639, t27821, t24600, t7301, t27798, t4935);
        let t94385 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2277::<F>(t24600, t24615, t1090, t12648, t14165, t2128, t24589, t24590, t24601, t24603, t27411, t27433, t27549, t27774, t4728, t5059, t7287, t85661, t85669, t86403, t94349, t94354, t94358, t94363, t94365, t94369, t94374);
        let (t94395, t94400, t94404, t94428) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2278::<F>(t24637, t8009, t24588, t8020, t1184, t4929, t1715, t3469, t24645, t7999, t1186, t1235, t15789, t1716, t1761, t24567, t24571, t24589, t24605, t24611, t24615, t27406, t27411, t27437, t27445, t27453, t27799, t460, t7283, t7286, t7300, t86403, t86475);
        let (t94436, t94439, t94446, t94451, t94456, t94458) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2279::<F>(t2121, t3427, t8010, t24574, t27416, t27794, t27441, t85639, t27446, t1751, t225, t461);
        let t94464 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2280::<F>(t11925, t14972, t24563, t24589, t24604, t24884, t27382, t27406, t27433, t27751, t3471, t7283, t7356, t8002, t8061, t85701, t85728, t86415, t94436, t94439, t94446, t94451, t94456, t94458);
        let (t94475, t94476, t94490) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2281::<F>(t24574, t27812, t8006, t85660, t23383, t8020);
        let t94498 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2282::<F>(t7303, t94490, t7291, t11605, t1186, t1251, t1761, t2155, t24589, t24601, t24602, t27761, t27766, t27784, t3487, t3966, t5059, t51937, t7283, t7391, t8002, t85711, t85717, t85724, t85733, t94475, t94476);
        let t94530 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2283::<F>(t2122, t94319, t8034, t8003, t85660, t1186, t11928, t15786, t24582, t24589, t24604, t24633, t27388, t27396, t27830, t3487, t3600, t5055, t7283, t7300, t7301, t8014, t8061, t85707, t85739, t85741, t85766);
        let t94564 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2284::<F>(t24574, t27412, t5052, t7299, t14972, t15359, t15790, t1716, t2123, t24596, t24601, t24617, t27381, t27396, t27406, t27549, t27820, t3243, t3593, t4930, t7283, t7295, t7302, t7351, t7392, t85787, t85789, t86452);
        let (t94588, t94605) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2285::<F>(t27381, t7294, t1715, t3475, t1186, t11928, t1238, t15802, t1760, t2155, t24589, t24597, t24603, t24615, t24616, t24867, t24897, t27406, t27437, t27549, t27751, t27761, t27775, t27799, t3477, t3593, t3598, t4723, t4945, t52386, t7283, t7300, t8010, t8088, t86403, t86415, t94369);
    (t94385, t94395, t94400, t94404, t94428, t94464, t94490, t94498, t94530, t94564, t94588, t94605)
}
