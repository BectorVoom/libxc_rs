//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta637 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2033;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2034;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2035;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2036;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2037;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2038;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2039;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta637(t531: f64, t7939: f64, t12550: f64, t12557: f64, t1442: f64, t15857: f64, t15904: f64, t1983: f64, t2036: f64, t22574: f64, t22584: f64, t22596: f64, t2314: f64, t2363: f64, t23938: f64, t24176: f64, t24428: f64, t24432: f64, t26161: f64, t26558: f64, t26905: f64, t26977: f64, t27219: f64, t33899: f64, t3929: f64, t4073: f64, t5107: f64, t56120: f64, t56194: f64, t652: f64, t7040: f64, t7042: f64, t7056: f64, t7685: f64, t7687: f64, t7890: f64, t7900: f64, t84347: f64, t90437: f64, t12734: f64, t12813: f64, t1458: f64, t16148: f64, t16153: f64, t16503: f64, t2040: f64, t2075: f64, t2079: f64, t23909: f64, t23958: f64, t24028: f64, t24987: f64, t24995: f64, t26114: f64, t26179: f64, t26559: f64, t27150: f64, t27226: f64, t4028: f64, t4034: f64, t4072: f64, t7050: f64, t7156: f64, t7170: f64, t7171: f64, t7802: f64, t90023: f64, t9016: f64, t90370: f64, t91669: f64, t91753: f64, t12725: f64, t12823: f64, t12841: f64, t1774: f64, t19456: f64, t2312: f64, t2364: f64, t23918: f64, t23929: f64, t24008: f64, t27188: f64, t4037: f64, t55962: f64, t57802: f64, t672: f64, t7057: f64, t7458: f64, t7796: f64, t92090: f64, t9348: f64, t2096: f64, t22578: f64, t22607: f64, t23953: f64, t24175: f64, t24442: f64, t24990: f64, t26878: f64, t26898: f64, t27163: f64, t3652: f64, t45632: f64, t5361: f64, t55934: f64, t6876: f64, t7166: f64, t7801: f64, t7806: f64, t7940: f64, t7941: f64, t86672: f64, t91565: f64, t91603: f64, t91695: f64, t92161: f64, t92210: f64, t93275: f64, t93930: f64, t1404: f64, t7945: f64, t2105: f64, t5363: f64, t2098: f64, t5381: f64, t27286: f64, t576: f64, t112: f64, t27240: f64, t12521: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t2039: f64, t23917: f64, t24462: f64, t24478: f64, t24481: f64, t27170: f64, t27254: f64, t27273: f64, t27276: f64, t3941: f64, t5371: f64, t5376: f64, t55353: f64, t55405: f64, t671: f64, t7235: f64, t84033: f64, t84078: f64, t92128: f64, t2319: f64, t111: f64, t16535: f64, t16538: f64, t16541: f64, t20173: f64, t24465: f64, t27281: f64, t3938: f64, t45560: f64, t55341: f64, t55571: f64, t577: f64, t66940: f64, t7230: f64, t7956: f64, t1398: f64, t16507: f64, t1858: f64, t24448: f64, t27241: f64, t3: f64, t3946: f64, t580: f64, t7946: f64, t85379: f64, t85381: f64, t85392: f64, t91846: f64) -> f64 {
        let t93978 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2032(t531, t7939, t12550, t12557, t1442, t15857, t15904, t1983, t2036, t22574, t22584, t22596, t2314, t2363, t23938, t24176, t24428, t24432, t26161, t26558, t26905, t26977, t27219, t33899, t3929, t4073, t5107, t56120, t56194, t652, t7040, t7042, t7056, t7685, t7687, t7890, t7900, t84347, t90437);
        let t94022 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2033(t12734, t12813, t1458, t16148, t16153, t16503, t1983, t2040, t2075, t2079, t2314, t23909, t23958, t24028, t24428, t24987, t24995, t26114, t26179, t26559, t27150, t27226, t4028, t4034, t4072, t652, t7050, t7156, t7170, t7171, t7685, t7802, t90023, t9016, t90370, t91669, t91753);
        let t94061 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2034(t12725, t12823, t12841, t1774, t19456, t2040, t22574, t2312, t2314, t2364, t23918, t23929, t23938, t24008, t26114, t26558, t27150, t27188, t27219, t27226, t4028, t4034, t4037, t55962, t57802, t672, t7042, t7050, t7057, t7458, t7796, t7802, t7890, t92090, t9348);
        let t94103 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2035(t12725, t12734, t1983, t2040, t2096, t22574, t22578, t22607, t2314, t23953, t24175, t24432, t24442, t24990, t24995, t26558, t26878, t26898, t27163, t3652, t4028, t45632, t5361, t55934, t652, t6876, t7050, t7166, t7685, t7796, t7801, t7806, t7940, t7941, t86672, t91565, t91603, t91695, t9348);
        let (t94106, t94113, t94118) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2036(t92161, t92210, t93275, t93930, t93978, t94022, t94061, t94103, t1404, t7945, t2105, t5363);
        let (t94120, t94122, t94160) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2037(t2098, t5381, t27286, t576, t112, t27240, t12521, t12524, t1401, t1458, t16521, t16524, t2039, t2363, t23917, t24462, t24478, t24481, t27170, t27254, t27273, t27276, t3941, t4072, t5371, t5376, t55353, t55405, t671, t7056, t7235, t7801, t84033, t84078, t92128);
        let t94202 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2038(t2098, t2319, t111, t7945, t12524, t12813, t1458, t16535, t16538, t16541, t20173, t2039, t23917, t24465, t27170, t27273, t27276, t27281, t3938, t3941, t4072, t45560, t55341, t55571, t577, t66940, t7056, t7230, t7801, t7956, t94106);
        let t94205 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2039(t1398, t1404, t16507, t1858, t2105, t24448, t27241, t3, t3946, t580, t7946, t85379, t85381, t85392, t94106, t94113, t94118, t94120, t94122, t94160, t94202);
        let tv4rho3sigma4 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2040(t91846, t94205);
    tv4rho3sigma4
}
