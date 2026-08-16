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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta679(t11153: f64, t497: f64, t27654: f64, t491: f64, t1235: f64, t8034: f64, t27434: f64, t85639: f64, t27821: f64, t24600: f64, t7301: f64, t27798: f64, t4935: f64, t24615: f64, t1090: f64, t12648: f64, t14165: f64, t2128: f64, t24589: f64, t24590: f64, t24601: f64, t24603: f64, t27411: f64, t27433: f64, t27549: f64, t27774: f64, t4728: f64, t5059: f64, t7287: f64, t85661: f64, t85669: f64, t86403: f64, t24637: f64, t8009: f64, t24588: f64, t8020: f64, t1184: f64, t4929: f64, t1715: f64, t3469: f64, t24645: f64, t7999: f64, t1186: f64, t15789: f64, t1716: f64, t1761: f64, t24567: f64, t24571: f64, t24605: f64, t24611: f64, t27406: f64, t27437: f64, t27445: f64, t27453: f64, t27799: f64, t460: f64, t7283: f64, t7286: f64, t7300: f64, t86475: f64, t2121: f64, t3427: f64, t8010: f64, t24574: f64, t27416: f64, t27794: f64, t27441: f64, t27446: f64, t1751: f64, t225: f64, t461: f64, t11925: f64, t14972: f64, t24563: f64, t24604: f64, t24884: f64, t27382: f64, t27751: f64, t3471: f64, t7356: f64, t8002: f64, t8061: f64, t85701: f64, t85728: f64, t86415: f64, t27812: f64, t8006: f64, t85660: f64, t23383: f64, t7303: f64, t7291: f64, t11605: f64, t1251: f64, t2155: f64, t24602: f64, t27761: f64, t27766: f64, t27784: f64, t3487: f64, t3966: f64, t51937: f64, t7391: f64, t85711: f64, t85717: f64, t85724: f64, t85733: f64, t2122: f64, t94319: f64, t8003: f64, t11928: f64, t15786: f64, t24582: f64, t24633: f64, t27388: f64, t27396: f64, t27830: f64, t3600: f64, t5055: f64, t8014: f64, t85707: f64, t85739: f64, t85741: f64, t85766: f64, t27412: f64, t5052: f64, t7299: f64, t15359: f64, t15790: f64, t2123: f64, t24596: f64, t24617: f64, t27381: f64, t27820: f64, t3243: f64, t3593: f64, t4930: f64, t7295: f64, t7302: f64, t7351: f64, t7392: f64, t85787: f64, t85789: f64, t86452: f64, t7294: f64, t3475: f64, t1238: f64, t15802: f64, t1760: f64, t24597: f64, t24616: f64, t24867: f64, t24897: f64, t27775: f64, t3477: f64, t3598: f64, t4723: f64, t4945: f64, t52386: f64, t8088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94349, t94354, t94358, t94363, t94365, t94369, t94374) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2276(t11153, t497, t27654, t491, t1235, t8034, t27434, t85639, t27821, t24600, t7301, t27798, t4935);
        let t94385 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2277(t24600, t24615, t1090, t12648, t14165, t2128, t24589, t24590, t24601, t24603, t27411, t27433, t27549, t27774, t4728, t5059, t7287, t85661, t85669, t86403, t94349, t94354, t94358, t94363, t94365, t94369, t94374);
        let (t94395, t94400, t94404, t94428) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2278(t24637, t8009, t24588, t8020, t1184, t4929, t1715, t3469, t24645, t7999, t1186, t1235, t15789, t1716, t1761, t24567, t24571, t24589, t24605, t24611, t24615, t27406, t27411, t27437, t27445, t27453, t27799, t460, t7283, t7286, t7300, t86403, t86475);
        let (t94436, t94439, t94446, t94451, t94456, t94458) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2279(t2121, t3427, t8010, t24574, t27416, t27794, t27441, t85639, t27446, t1751, t225, t461);
        let t94464 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2280(t11925, t14972, t24563, t24589, t24604, t24884, t27382, t27406, t27433, t27751, t3471, t7283, t7356, t8002, t8061, t85701, t85728, t86415, t94436, t94439, t94446, t94451, t94456, t94458);
        let (t94475, t94476, t94490) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2281(t24574, t27812, t8006, t85660, t23383, t8020);
        let t94498 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2282(t7303, t94490, t7291, t11605, t1186, t1251, t1761, t2155, t24589, t24601, t24602, t27761, t27766, t27784, t3487, t3966, t5059, t51937, t7283, t7391, t8002, t85711, t85717, t85724, t85733, t94475, t94476);
        let t94530 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2283(t2122, t94319, t8034, t8003, t85660, t1186, t11928, t15786, t24582, t24589, t24604, t24633, t27388, t27396, t27830, t3487, t3600, t5055, t7283, t7300, t7301, t8014, t8061, t85707, t85739, t85741, t85766);
        let t94564 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2284(t24574, t27412, t5052, t7299, t14972, t15359, t15790, t1716, t2123, t24596, t24601, t24617, t27381, t27396, t27406, t27549, t27820, t3243, t3593, t4930, t7283, t7295, t7302, t7351, t7392, t85787, t85789, t86452);
        let (t94588, t94605) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2285(t27381, t7294, t1715, t3475, t1186, t11928, t1238, t15802, t1760, t2155, t24589, t24597, t24603, t24615, t24616, t24867, t24897, t27406, t27437, t27549, t27751, t27761, t27775, t27799, t3477, t3593, t3598, t4723, t4945, t52386, t7283, t7300, t8010, t8088, t86403, t86415, t94369);
    (t94385, t94395, t94400, t94404, t94428, t94464, t94490, t94498, t94530, t94564, t94588, t94605)
}
