//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta645 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2209;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta645(t23422: f64, t4603: f64, t14159: f64, t6717: f64, t14137: f64, t6765: f64, t7583: f64, t83138: f64, t23509: f64, t25682: f64, t25644: f64, t82926: f64, t23512: f64, t25486: f64, t23519: f64, t25492: f64, t1607: f64, t23515: f64, t23521: f64, t23529: f64, t4636: f64, t6747: f64, t82911: f64, t82951: f64, t82953: f64, t83092: f64, t1597: f64, t607: f64, t23562: f64, t343: f64, t40: f64, t4540: f64, t25650: f64, t6740: f64, t14206: f64, t6754: f64, t12606: f64, t3: f64, t1025: f64, t1933: f64, t1937: f64, t23453: f64, t23504: f64, t25588: f64, t25645: f64, t6722: f64, t7573: f64, t82927: f64, t82961: f64, t83111: f64, t25651: f64, t83120: f64, t1409: f64, t984: f64, t1036: f64, t25622: f64, t14134: f64, t23479: f64, t25637: f64, t1014: f64, t82654: f64, t1022: f64, t14037: f64, t1611: f64, t23419: f64, t23556: f64, t25655: f64, t25661: f64, t363: f64, t378: f64, t6800: f64, t82971: f64, t82996: f64, t83085: f64, t344: f64, t1009: f64, t23473: f64, t13528: f64, t13542: f64, t13931: f64, t14130: f64, t1618: f64, t1920: f64, t1934: f64, t1935: f64, t23414: f64, t23495: f64, t25601: f64, t25609: f64, t2987: f64, t4509: f64, t6730: f64, t6734: f64, t6735: f64, t7578: f64, t82880: f64, t83004: f64, t83025: f64, t83028: f64, t3082: f64, t7586: f64, t25641: f64, t82892: f64, t25638: f64, t13532: f64, t13537: f64, t13797: f64, t13941: f64, t14122: f64, t14126: f64, t1941: f64, t23548: f64, t23564: f64, t25679: f64, t7574: f64, t82918: f64, t82923: f64, t83016: f64, t83034: f64, t83215: f64, t23418: f64, t4669: f64, t13765: f64, t14033: f64, t14069: f64, t14488: f64, t23457: f64, t25585: f64, t25589: f64, t3073: f64, t360: f64, t4575: f64, t6723: f64, t6742: f64, t6744: f64, t68: f64, t83041: f64, t83046: f64, t83220: f64, t10469: f64, t23470: f64, t82986: f64, t23437: f64, t4630: f64, t82943: f64, t3966: f64, t14222: f64, t1622: f64, t23544: f64, t25580: f64, t25600: f64, t25658: f64, t3032: f64, t3040: f64, t3098: f64, t4579: f64, t6729: f64, t83071: f64, t83075: f64, t82895: f64, t25664: f64, t23528: f64, t23436: f64, t4640: f64, t14507: f64, t23536: f64, t1046: f64, t25683: f64, t3057: f64, t3134: f64, t4616: f64, t6758: f64, t82868: f64, t83080: f64, t83082: f64, t83098: f64, t23540: f64, t23433: f64, t10189: f64, t4343: f64, t13783: f64, t4338: f64, t13546: f64, t13555: f64, t13559: f64, t14099: f64, t14103: f64, t14167: f64, t23541: f64, t25571: f64, t25574: f64, t3043: f64, t6680: f64, t82964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88335, t88336, t88339, t88341, t88342, t88348) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207(t23422, t4603, t14159, t6717, t14137, t6765, t7583, t83138, t23509, t25682, t25644, t82926);
        let t88358 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208(t23512, t25486, t23519, t25492, t1607, t23515, t23521, t23529, t4636, t6747, t82911, t82951, t82953, t83092, t88335, t88336, t88339, t88341, t88342, t88348);
        let (t88360, t88362, t88365, t88367, t88372, t88385, t88388) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2209(t1597, t607, t23562, t343, t40, t4540, t25644, t25650, t6740, t6747, t14206, t6754);
        let t88397 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210(t12606, t3, t1025, t1933, t1937, t23453, t23504, t23515, t23521, t25588, t25645, t6722, t6747, t7573, t7583, t82927, t82961, t83111, t88362, t88367, t88372, t88385, t88388);
        let (t88400, t88405, t88407, t88415, t88422, t88425) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211(t25651, t3, t83120, t1409, t984, t23562, t343, t1036, t25622, t14134, t6765, t1933, t23479, t88360);
        let t88437 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212(t1933, t23479, t88365, t23562, t25637, t984, t1014, t82654, t1022, t14037, t1611, t23419, t23556, t25655, t25661, t363, t378, t6747, t6800, t7583, t82971, t82996, t83085, t88400, t88407, t88415, t88422, t88425);
        let t88472 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213(t23479, t25637, t6722, t1409, t344, t1009, t6740, t23473, t13528, t13542, t13931, t14130, t1618, t1920, t1933, t1934, t1935, t23414, t23419, t23495, t25601, t25609, t2987, t343, t4509, t4540, t6730, t6734, t6735, t7578, t82880, t83004, t83025, t83028);
        let t88504 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214(t3082, t7586, t25641, t82892, t25638, t6735, t13532, t13537, t13797, t13941, t14122, t14126, t1920, t1941, t23548, t23564, t25679, t378, t4509, t7574, t7583, t82918, t82923, t83016, t83034, t83215);
        let t88533 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215(t23418, t4669, t13765, t23419, t14033, t14069, t14488, t23457, t23495, t25585, t25589, t25609, t3073, t360, t4575, t6723, t6735, t6742, t6744, t68, t7574, t7578, t83041, t83046, t83220);
        let (t88537, t88570) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216(t10469, t23470, t3, t82986, t23437, t4630, t25641, t82943, t1933, t1937, t3966, t14222, t1597, t1622, t23544, t23548, t25580, t25600, t25601, t25658, t3032, t3040, t3098, t360, t4579, t4636, t6722, t6729, t6735, t83071, t83075, t83215, t83220);
        let t88597 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217(t25655, t82895, t25661, t1036, t25664, t1611, t23528, t23436, t4640, t14507, t23536, t1025, t1046, t1622, t23504, t25580, t25683, t3057, t3134, t378, t4616, t6758, t82868, t83080, t83082, t83098);
        let t88632 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218(t14507, t23540, t23433, t4630, t10189, t1920, t4343, t13783, t4338, t13546, t13555, t13559, t14099, t14103, t14167, t1618, t23541, t25571, t25574, t2987, t3043, t4509, t6680, t6765, t82964);
    (t88358, t88397, t88405, t88437, t88472, t88504, t88533, t88537, t88570, t88597, t88632)
}
