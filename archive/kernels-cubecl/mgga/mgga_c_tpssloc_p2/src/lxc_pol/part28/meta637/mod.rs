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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta637<F: Float>(t531: F, t7939: F, t12550: F, t12557: F, t1442: F, t15857: F, t15904: F, t1983: F, t2036: F, t22574: F, t22584: F, t22596: F, t2314: F, t2363: F, t23938: F, t24176: F, t24428: F, t24432: F, t26161: F, t26558: F, t26905: F, t26977: F, t27219: F, t33899: F, t3929: F, t4073: F, t5107: F, t56120: F, t56194: F, t652: F, t7040: F, t7042: F, t7056: F, t7685: F, t7687: F, t7890: F, t7900: F, t84347: F, t90437: F, t12734: F, t12813: F, t1458: F, t16148: F, t16153: F, t16503: F, t2040: F, t2075: F, t2079: F, t23909: F, t23958: F, t24028: F, t24987: F, t24995: F, t26114: F, t26179: F, t26559: F, t27150: F, t27226: F, t4028: F, t4034: F, t4072: F, t7050: F, t7156: F, t7170: F, t7171: F, t7802: F, t90023: F, t9016: F, t90370: F, t91669: F, t91753: F, t12725: F, t12823: F, t12841: F, t1774: F, t19456: F, t2312: F, t2364: F, t23918: F, t23929: F, t24008: F, t27188: F, t4037: F, t55962: F, t57802: F, t672: F, t7057: F, t7458: F, t7796: F, t92090: F, t9348: F, t2096: F, t22578: F, t22607: F, t23953: F, t24175: F, t24442: F, t24990: F, t26878: F, t26898: F, t27163: F, t3652: F, t45632: F, t5361: F, t55934: F, t6876: F, t7166: F, t7801: F, t7806: F, t7940: F, t7941: F, t86672: F, t91565: F, t91603: F, t91695: F, t92161: F, t92210: F, t93275: F, t93930: F, t1404: F, t7945: F, t2105: F, t5363: F, t2098: F, t5381: F, t27286: F, t576: F, t112: F, t27240: F, t12521: F, t12524: F, t1401: F, t16521: F, t16524: F, t2039: F, t23917: F, t24462: F, t24478: F, t24481: F, t27170: F, t27254: F, t27273: F, t27276: F, t3941: F, t5371: F, t5376: F, t55353: F, t55405: F, t671: F, t7235: F, t84033: F, t84078: F, t92128: F, t2319: F, t111: F, t16535: F, t16538: F, t16541: F, t20173: F, t24465: F, t27281: F, t3938: F, t45560: F, t55341: F, t55571: F, t577: F, t66940: F, t7230: F, t7956: F, t1398: F, t16507: F, t1858: F, t24448: F, t27241: F, t3: F, t3946: F, t580: F, t7946: F, t85379: F, t85381: F, t85392: F, t91846: F) -> F {
        let t93978 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2032::<F>(t531, t7939, t12550, t12557, t1442, t15857, t15904, t1983, t2036, t22574, t22584, t22596, t2314, t2363, t23938, t24176, t24428, t24432, t26161, t26558, t26905, t26977, t27219, t33899, t3929, t4073, t5107, t56120, t56194, t652, t7040, t7042, t7056, t7685, t7687, t7890, t7900, t84347, t90437);
        let t94022 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2033::<F>(t12734, t12813, t1458, t16148, t16153, t16503, t1983, t2040, t2075, t2079, t2314, t23909, t23958, t24028, t24428, t24987, t24995, t26114, t26179, t26559, t27150, t27226, t4028, t4034, t4072, t652, t7050, t7156, t7170, t7171, t7685, t7802, t90023, t9016, t90370, t91669, t91753);
        let t94061 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2034::<F>(t12725, t12823, t12841, t1774, t19456, t2040, t22574, t2312, t2314, t2364, t23918, t23929, t23938, t24008, t26114, t26558, t27150, t27188, t27219, t27226, t4028, t4034, t4037, t55962, t57802, t672, t7042, t7050, t7057, t7458, t7796, t7802, t7890, t92090, t9348);
        let t94103 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2035::<F>(t12725, t12734, t1983, t2040, t2096, t22574, t22578, t22607, t2314, t23953, t24175, t24432, t24442, t24990, t24995, t26558, t26878, t26898, t27163, t3652, t4028, t45632, t5361, t55934, t652, t6876, t7050, t7166, t7685, t7796, t7801, t7806, t7940, t7941, t86672, t91565, t91603, t91695, t9348);
        let (t94106, t94113, t94118) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2036::<F>(t92161, t92210, t93275, t93930, t93978, t94022, t94061, t94103, t1404, t7945, t2105, t5363);
        let (t94120, t94122, t94160) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2037::<F>(t2098, t5381, t27286, t576, t112, t27240, t12521, t12524, t1401, t1458, t16521, t16524, t2039, t2363, t23917, t24462, t24478, t24481, t27170, t27254, t27273, t27276, t3941, t4072, t5371, t5376, t55353, t55405, t671, t7056, t7235, t7801, t84033, t84078, t92128);
        let t94202 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2038::<F>(t2098, t2319, t111, t7945, t12524, t12813, t1458, t16535, t16538, t16541, t20173, t2039, t23917, t24465, t27170, t27273, t27276, t27281, t3938, t3941, t4072, t45560, t55341, t55571, t577, t66940, t7056, t7230, t7801, t7956, t94106);
        let t94205 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2039::<F>(t1398, t1404, t16507, t1858, t2105, t24448, t27241, t3, t3946, t580, t7946, t85379, t85381, t85392, t94106, t94113, t94118, t94120, t94122, t94160, t94202);
        let tv4rho3sigma4 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2040::<F>(t91846, t94205);
    tv4rho3sigma4
}
