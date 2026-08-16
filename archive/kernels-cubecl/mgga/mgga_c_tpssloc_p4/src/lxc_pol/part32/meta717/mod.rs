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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta717<F: Float>(t23164: F, t7479: F, t86893: F, t17063: F, t23278: F, t25168: F, t5637: F, t82294: F, t87748: F, t87902: F, t87911: F, t87927: F, t87932: F, t92954: F, t92961: F, t99033: F, t10109: F, t13042: F, t13463: F, t1528: F, t16804: F, t17050: F, t17057: F, t17064: F, t17069: F, t17070: F, t17090: F, t17092: F, t1902: F, t1912: F, t23281: F, t25169: F, t25170: F, t25184: F, t25188: F, t25200: F, t25233: F, t25348: F, t259: F, t2713: F, t2718: F, t28307: F, t28311: F, t28431: F, t4147: F, t4268: F, t4272: F, t4273: F, t4301: F, t5558: F, t5657: F, t5658: F, t59498: F, t59503: F, t59537: F, t6624: F, t6627: F, t6632: F, t6662: F, t6663: F, t7517: F, t7537: F, t7538: F, t82087: F, t82099: F, t855: F, t865: F, t866: F, t86903: F, t86941: F, t86943: F, t87758: F, t87777: F, t87787: F, t87797: F, t87810: F, t87836: F, t87837: F, t87874: F, t92402: F, t92863: F, t98160: F, t98164: F, t98166: F, t98172: F, t98181: F, t98208: F, t98213: F, t98222: F, t98227: F, t98258: F, t98264: F, t98277: F, t98279: F, t98309: F, t98913: F, t98921: F, t98923: F, t98947: F, t98963: F, t98966: F, t98999: F, t99003: F, t99010: F, t99019: F, t99022: F, t870: F, t16596: F, t86721: F, t1484: F, t584: F, t86753: F, t22959: F, t16949: F, t25014: F, t1408: F, t4255: F, t193: F, t200: F, t7540: F, t16557: F, t1877: F, t1915: F, t23295: F, t25: F, t25013: F, t25015: F, t25021: F, t2522: F, t25354: F, t25366: F, t25372: F, t25385: F, t7541: F, t86736: F, t98091: F, t98094: F, t98103: F, t98112: F, t97989: F, t98039: F, t98090: F, t16558: F, t3: F, t25365: F, t57911: F, t10143: F, t25374: F, t16944: F, t202: F, t23290: F, t25358: F, t28248: F, t4314: F, t5544: F, t6666: F, t6670: F, t67128: F, t82312: F, t97999: F, t98003: F, t98007: F, t98011: F, t1530: F, t16662: F, t17109: F, t28448: F, t28732: F, t4119: F, t4303: F, t46341: F, t5527: F, t5660: F, t5664: F, t67123: F, t67164: F, t776: F, t81539: F, t868: F, t86836: F, t87975: F, t98030: F, t98054: F, t98102: F, t23788: F, t25891: F, t25927: F, t5966: F, t1649: F, t83555: F, t1081: F, t25892: F, t25921: F, t28771: F, t81483: F, t97972: F, t89953: F, t98111: F, t18196: F, t25898: F, t25945: F, t28: F, t28778: F, t28789: F, t6848: F, t98071: F, t25901: F, t25905: F, t25928: F, t25938: F, t28764: F, t28765: F, t6841: F, t98027: F, t89992: F, t98058: F, t25930: F, t25934: F, t28774: F, t28792: F, t28795: F, t7649: F, t7656: F) -> (F, F, F, F, F, F, F) {
        let t99038 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2271::<F>(t23164, t7479, t86893, t17063, t23278, t25168, t5637, t82294, t87748, t87902, t87911, t87927, t87932, t92954, t92961, t99033);
        let t99042 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2272::<F>(t10109, t13042, t13463, t1528, t16804, t17050, t17057, t17064, t17069, t17070, t17090, t17092, t1902, t1912, t23278, t23281, t25168, t25169, t25170, t25184, t25188, t25200, t25233, t25348, t259, t2713, t2718, t28307, t28311, t28431, t4147, t4268, t4272, t4273, t4301, t5558, t5637, t5657, t5658, t59498, t59503, t59537, t6624, t6627, t6632, t6662, t6663, t7517, t7537, t7538, t82087, t82099, t855, t865, t866, t86903, t86941, t86943, t87758, t87777, t87787, t87797, t87810, t87836, t87837, t87874, t92402, t92863, t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98258, t98264, t98277, t98279, t98309, t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99038);
        let (t99043, t99049, t99055, t99056, t99060) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2273::<F>(t870, t99042, t16596, t86721, t1484, t584, t86753, t22959, t16949, t25014, t1408, t4255);
        let (t99064, t99067) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2274::<F>(t193, t200, t7540, t1408, t16557, t1877, t1915, t22959, t23295, t25, t25013, t25015, t25021, t2522, t25354, t25366, t25372, t25385, t7541, t86736, t98091, t98094, t98103, t98112, t99043, t99049, t99055, t99056, t99060);
        let (t99069, t99767, t100578) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2275::<F>(t97989, t98039, t98090, t99067, t16558, t3, t25365, t57911, t10143, t1484, t25374, t16596, t16944, t16949, t1877, t1915, t193, t202, t22959, t23290, t23295, t25013, t2522, t25354, t25358, t28248, t4255, t4314, t5544, t6666, t6670, t67128, t7541, t82312, t870, t97999, t98003, t98007, t98011, t99042);
        let t100623 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2276::<F>(t1530, t16662, t17109, t1877, t1915, t23290, t23295, t2522, t25358, t25374, t28448, t28732, t4119, t4303, t4314, t46341, t5527, t5660, t5664, t6666, t6670, t67123, t67164, t7541, t776, t81539, t868, t86836, t87975, t98030, t98054, t98102);
        let (t100624, t100638, t100641, t100644, t100646, t100651) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2277::<F>(t100578, t100623, t23788, t67128, t16949, t25891, t25927, t98102, t5966, t868, t1649, t4255, t870);
        let t100674 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2278::<F>(t28248, t83555, t25927, t98030, t23788, t98011, t1081, t5664, t100638, t100641, t100644, t100646, t100651, t1649, t1877, t22959, t23295, t25013, t25354, t25372, t25892, t25921, t28771, t6670, t81483, t86736, t97972, t99064);
        let (t100682, t100689, t100692, t100696, t100705, t100708) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2279::<F>(t89953, t97999, t10143, t1649, t25374, t5966, t776, t4303, t23788, t67164, t16944, t25891);
        let t100716 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2280::<F>(t25927, t98111, t100682, t100689, t100692, t100696, t100705, t100708, t18196, t1877, t1915, t22959, t25013, t2522, t25358, t25372, t25898, t25945, t28, t28778, t28789, t6666, t6670, t6848, t81539, t86736, t98054, t98071, t99043);
        let t100763 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2281::<F>(t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28, t5527, t1877, t1915, t22959, t2522, t25901, t25905, t25928, t25938, t28448, t28764, t28765, t4314, t46341, t5966, t6666, t6670, t6841, t7541, t98027);
        let t100803 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2282::<F>(t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003, t1081, t1877, t22959, t23290, t25013, t2522, t25354, t25358, t25930, t25934, t28448, t28774, t28792, t28795, t6666, t6670, t7649, t7656, t86836, t99055);
    (t99069, t99767, t100624, t100674, t100716, t100763, t100803)
}
