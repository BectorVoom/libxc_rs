//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta717 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2271;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2272;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2273;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2274;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2275;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2276;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2277;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2278;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2279;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2280;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2281;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta717(t23164: f64, t7479: f64, t86893: f64, t17063: f64, t23278: f64, t25168: f64, t5637: f64, t82294: f64, t87748: f64, t87902: f64, t87911: f64, t87927: f64, t87932: f64, t92954: f64, t92961: f64, t99033: f64, t10109: f64, t13042: f64, t13463: f64, t1528: f64, t16804: f64, t17050: f64, t17057: f64, t17064: f64, t17069: f64, t17070: f64, t17090: f64, t17092: f64, t1902: f64, t1912: f64, t23281: f64, t25169: f64, t25170: f64, t25184: f64, t25188: f64, t25200: f64, t25233: f64, t25348: f64, t259: f64, t2713: f64, t2718: f64, t28307: f64, t28311: f64, t28431: f64, t4147: f64, t4268: f64, t4272: f64, t4273: f64, t4301: f64, t5558: f64, t5657: f64, t5658: f64, t59498: f64, t59503: f64, t59537: f64, t6624: f64, t6627: f64, t6632: f64, t6662: f64, t6663: f64, t7517: f64, t7537: f64, t7538: f64, t82087: f64, t82099: f64, t855: f64, t865: f64, t866: f64, t86903: f64, t86941: f64, t86943: f64, t87758: f64, t87777: f64, t87787: f64, t87797: f64, t87810: f64, t87836: f64, t87837: f64, t87874: f64, t92402: f64, t92863: f64, t98160: f64, t98164: f64, t98166: f64, t98172: f64, t98181: f64, t98208: f64, t98213: f64, t98222: f64, t98227: f64, t98258: f64, t98264: f64, t98277: f64, t98279: f64, t98309: f64, t98913: f64, t98921: f64, t98923: f64, t98947: f64, t98963: f64, t98966: f64, t98999: f64, t99003: f64, t99010: f64, t99019: f64, t99022: f64, t870: f64, t16596: f64, t86721: f64, t1484: f64, t584: f64, t86753: f64, t22959: f64, t16949: f64, t25014: f64, t1408: f64, t4255: f64, t193: f64, t200: f64, t7540: f64, t16557: f64, t1877: f64, t1915: f64, t23295: f64, t25: f64, t25013: f64, t25015: f64, t25021: f64, t2522: f64, t25354: f64, t25366: f64, t25372: f64, t25385: f64, t7541: f64, t86736: f64, t98091: f64, t98094: f64, t98103: f64, t98112: f64, t97989: f64, t98039: f64, t98090: f64, t16558: f64, t3: f64, t25365: f64, t57911: f64, t10143: f64, t25374: f64, t16944: f64, t202: f64, t23290: f64, t25358: f64, t28248: f64, t4314: f64, t5544: f64, t6666: f64, t6670: f64, t67128: f64, t82312: f64, t97999: f64, t98003: f64, t98007: f64, t98011: f64, t1530: f64, t16662: f64, t17109: f64, t28448: f64, t28732: f64, t4119: f64, t4303: f64, t46341: f64, t5527: f64, t5660: f64, t5664: f64, t67123: f64, t67164: f64, t776: f64, t81539: f64, t868: f64, t86836: f64, t87975: f64, t98030: f64, t98054: f64, t98102: f64, t23788: f64, t25891: f64, t25927: f64, t5966: f64, t1649: f64, t83555: f64, t1081: f64, t25892: f64, t25921: f64, t28771: f64, t81483: f64, t97972: f64, t89953: f64, t98111: f64, t18196: f64, t25898: f64, t25945: f64, t28: f64, t28778: f64, t28789: f64, t6848: f64, t98071: f64, t25901: f64, t25905: f64, t25928: f64, t25938: f64, t28764: f64, t28765: f64, t6841: f64, t98027: f64, t89992: f64, t98058: f64, t25930: f64, t25934: f64, t28774: f64, t28792: f64, t28795: f64, t7649: f64, t7656: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t99038 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2271(t23164, t7479, t86893, t17063, t23278, t25168, t5637, t82294, t87748, t87902, t87911, t87927, t87932, t92954, t92961, t99033);
        let t99042 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2272(t10109, t13042, t13463, t1528, t16804, t17050, t17057, t17064, t17069, t17070, t17090, t17092, t1902, t1912, t23278, t23281, t25168, t25169, t25170, t25184, t25188, t25200, t25233, t25348, t259, t2713, t2718, t28307, t28311, t28431, t4147, t4268, t4272, t4273, t4301, t5558, t5637, t5657, t5658, t59498, t59503, t59537, t6624, t6627, t6632, t6662, t6663, t7517, t7537, t7538, t82087, t82099, t855, t865, t866, t86903, t86941, t86943, t87758, t87777, t87787, t87797, t87810, t87836, t87837, t87874, t92402, t92863, t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98258, t98264, t98277, t98279, t98309, t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99038);
        let (t99043, t99049, t99055, t99056, t99060) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2273(t870, t99042, t16596, t86721, t1484, t584, t86753, t22959, t16949, t25014, t1408, t4255);
        let (t99064, t99067) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2274(t193, t200, t7540, t1408, t16557, t1877, t1915, t22959, t23295, t25, t25013, t25015, t25021, t2522, t25354, t25366, t25372, t25385, t7541, t86736, t98091, t98094, t98103, t98112, t99043, t99049, t99055, t99056, t99060);
        let (t99069, t99767, t100578) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2275(t97989, t98039, t98090, t99067, t16558, t3, t25365, t57911, t10143, t1484, t25374, t16596, t16944, t16949, t1877, t1915, t193, t202, t22959, t23290, t23295, t25013, t2522, t25354, t25358, t28248, t4255, t4314, t5544, t6666, t6670, t67128, t7541, t82312, t870, t97999, t98003, t98007, t98011, t99042);
        let t100623 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2276(t1530, t16662, t17109, t1877, t1915, t23290, t23295, t2522, t25358, t25374, t28448, t28732, t4119, t4303, t4314, t46341, t5527, t5660, t5664, t6666, t6670, t67123, t67164, t7541, t776, t81539, t868, t86836, t87975, t98030, t98054, t98102);
        let (t100624, t100638, t100641, t100644, t100646, t100651) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2277(t100578, t100623, t23788, t67128, t16949, t25891, t25927, t98102, t5966, t868, t1649, t4255, t870);
        let t100674 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2278(t28248, t83555, t25927, t98030, t23788, t98011, t1081, t5664, t100638, t100641, t100644, t100646, t100651, t1649, t1877, t22959, t23295, t25013, t25354, t25372, t25892, t25921, t28771, t6670, t81483, t86736, t97972, t99064);
        let (t100682, t100689, t100692, t100696, t100705, t100708) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2279(t89953, t97999, t10143, t1649, t25374, t5966, t776, t4303, t23788, t67164, t16944, t25891);
        let t100716 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2280(t25927, t98111, t100682, t100689, t100692, t100696, t100705, t100708, t18196, t1877, t1915, t22959, t25013, t2522, t25358, t25372, t25898, t25945, t28, t28778, t28789, t6666, t6670, t6848, t81539, t86736, t98054, t98071, t99043);
        let t100763 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2281(t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28, t5527, t1877, t1915, t22959, t2522, t25901, t25905, t25928, t25938, t28448, t28764, t28765, t4314, t46341, t5966, t6666, t6670, t6841, t7541, t98027);
        let t100803 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2282(t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003, t1081, t1877, t22959, t23290, t25013, t2522, t25354, t25358, t25930, t25934, t28448, t28774, t28792, t28795, t6666, t6670, t7649, t7656, t86836, t99055);
    (t99069, t99767, t100624, t100674, t100716, t100763, t100803)
}
