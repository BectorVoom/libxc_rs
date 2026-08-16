//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta747 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2488;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2489;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2490;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2491;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2492;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2493;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2494;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2495;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2496;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2497;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2498;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta747<F: Float>(t13969: F, t21486: F, t3130: F, t1041: F, t13995: F, t17705: F, t17976: F, t18036: F, t21512: F, t3117: F, t43219: F, t4582: F, t4588: F, t4644: F, t49929: F, t50175: F, t50181: F, t62631: F, t62640: F, t70316: F, t2979: F, t43248: F, t50259: F, t50263: F, t62657: F, t62660: F, t62663: F, t62666: F, t62682: F, t62687: F, t68462: F, t68481: F, t973: F, t10422: F, t21565: F, t3070: F, t10403: F, t10937: F, t14172: F, t17998: F, t21391: F, t21566: F, t3071: F, t42388: F, t43253: F, t4347: F, t5873: F, t62704: F, t62766: F, t62778: F, t62780: F, t70339: F, t884: F, t21126: F, t2970: F, t1023: F, t1031: F, t17677: F, t21130: F, t21482: F, t21490: F, t21493: F, t2960: F, t378: F, t42397: F, t43307: F, t4579: F, t50362: F, t61950: F, t62811: F, t62816: F, t21569: F, t42488: F, t10408: F, t17156: F, t18014: F, t4338: F, t4343: F, t48607: F, t50324: F, t5677: F, t5867: F, t5909: F, t62827: F, t62832: F, t62836: F, t62840: F, t69742: F, t70241: F, t10231: F, t21122: F, t17649: F, t17681: F, t21526: F, t42541: F, t43382: F, t50425: F, t50443: F, t62891: F, t62893: F, t62901: F, t62903: F, t1020: F, t1021: F, t10214: F, t1022: F, t10413: F, t1044: F, t10480: F, t10482: F, t10883: F, t10891: F, t14080: F, t14085: F, t14187: F, t14220: F, t14511: F, t1616: F, t1618: F, t17632: F, t17670: F, t17671: F, t17688: F, t17693: F, t17923: F, t18016: F, t18025: F, t18030: F, t21393: F, t21398: F, t21503: F, t21542: F, t21546: F, t21603: F, t248: F, t2986: F, t360: F, t42334: F, t42561: F, t42861: F, t43157: F, t43291: F, t43292: F, t43322: F, t43385: F, t4342: F, t4583: F, t4649: F, t4650: F, t4652: F, t48585: F, t49651: F, t49771: F, t49819: F, t49832: F, t50078: F, t50370: F, t5861: F, t5875: F, t5878: F, t5880: F, t5900: F, t607: F, t61655: F, t61699: F, t61705: F, t61708: F, t61713: F, t61715: F, t61731: F, t61742: F, t61853: F, t62049: F, t62085: F, t62099: F, t62534: F, t62556: F, t62559: F, t62565: F, t62845: F, t67060: F, t68477: F, t68513: F, t68525: F, t68569: F, t69966: F, t70086: F, t70100: F, t70106: F, t70122: F, t70151: F, t70189: F, t70211: F, t70214: F, t70227: F, t70268: F, t70273: F, t70296: F, t70321: F, t70335: F, t70346: F, t70351: F, t70363: F, t70396: F, t70414: F, t70432: F, t70442: F, t70458: F, t70481: F, t70509: F, t70539: F, t70554: F, t70599: F, t70623: F, t70645: F, t70655: F, t70660: F, t70665: F, t70707: F, t70728: F, t70756: F, t70766: F, t70802: F, t974: F, t998: F, t1003: F, t1058: F, t1060: F, t11046: F, t11048: F, t14618: F, t14651: F, t18099: F, t18121: F, t18155: F, t21615: F, t21622: F, t21626: F, t3200: F, t4615: F, t4657: F, t4669: F, t4684: F, t4691: F, t50592: F, t5866: F, t5903: F, t5937: F, t5939: F, t5941: F, t70014: F, t21689: F, t225: F, t21669: F, t10165: F, t1052: F, t1055: F, t1066: F, t11037: F, t11065: F, t11066: F, t14545: F, t14555: F, t14608: F, t1615: F, t1625: F, t1629: F, t1630: F, t17575: F, t17959: F, t18047: F, t18062: F, t18081: F, t18086: F, t18093: F, t18104: F, t18107: F, t18108: F, t18112: F, t18139: F, t18142: F, t18151: F, t18161: F, t21617: F, t21623: F, t21627: F, t21634: F, t21635: F, t21637: F, t21638: F, t21643: F, t21653: F, t21656: F, t21692: F, t3026: F, t3174: F, t3180: F, t3186: F, t3188: F, t353: F, t381: F, t383: F, t384: F, t388: F, t43515: F, t43516: F, t43562: F, t4552: F, t4660: F, t4665: F, t4673: F, t4674: F, t4677: F, t4680: F, t4681: F, t4685: F, t4689: F, t4693: F, t47841: F, t47857: F, t50465: F, t50516: F, t50610: F, t5914: F, t5919: F, t5920: F, t5928: F, t5929: F, t5932: F, t5933: F, t5936: F, t5943: F, t5944: F, t62994: F, t63004: F, t63183: F, t69871: F, t69942: F, t70009: F, t70012: F, t70068: F, t70081: F, t70082: F, t21684: F, t14529: F, t1635: F, t18061: F, t18074: F, t18166: F, t21663: F, t25757: F, t3169: F, t4557: F, t4694: F, t50628: F, t5848: F, t61646: F, t63215: F, t1065: F, t14552: F, t1603: F, t1634: F, t18165: F, t21614: F, t21676: F, t21677: F, t349: F, t43604: F, t60971: F, t61061: F, t61621: F, t990: F, t1070: F, t193: F, t336: F, t69335: F, t69337: F, t69340: F, t69343: F, t69346: F, t69350: F, t69353: F, t69357: F, t69469: F, t69471: F, t69860: F) -> F {
        let t70823 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2488::<F>(t13969, t21486, t3130, t1041, t13995, t17705, t17976, t18036, t21512, t3117, t43219, t4582, t4588, t4644, t49929, t50175, t50181, t62631, t62640, t70316);
        let t70837 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2489::<F>(t2979, t43248, t50259, t50263, t62657, t62660, t62663, t62666, t62682, t62687, t68462, t68481, t973);
        let t70863 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2490::<F>(t10422, t21565, t3070, t10403, t1041, t10937, t13995, t14172, t17998, t21391, t21566, t3071, t42388, t43253, t4347, t4582, t5873, t62704, t62766, t62778, t62780, t70339, t884);
        let t70884 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2491::<F>(t21126, t2970, t973, t1023, t1031, t13995, t17677, t21130, t21482, t21490, t21493, t2960, t3070, t378, t42397, t43307, t4579, t50362, t61950, t62811, t62816);
        let t70917 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2492::<F>(t21569, t3070, t42488, t10403, t10408, t17156, t18014, t3071, t4338, t4343, t48607, t50324, t5677, t5867, t5909, t62827, t62832, t62836, t62840, t69742, t70241);
        let t70933 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2493::<F>(t10231, t21122, t973, t13995, t17649, t17681, t21526, t42541, t43382, t50425, t50443, t62891, t62893, t62901, t62903);
        let t70938 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2494::<F>(t1020, t1021, t10214, t1022, t10403, t10408, t1041, t10413, t1044, t10480, t10482, t10883, t10891, t13995, t14080, t14085, t14187, t14220, t14511, t1616, t1618, t17632, t17670, t17671, t17688, t17693, t17923, t18016, t18025, t18030, t21393, t21398, t21503, t21542, t21546, t21603, t248, t2960, t2986, t3070, t3071, t3117, t360, t42334, t42561, t42861, t43157, t43291, t43292, t43322, t4338, t43385, t4342, t4343, t4582, t4583, t4588, t4644, t4649, t4650, t4652, t48585, t49651, t49771, t49819, t49832, t49929, t50078, t50370, t5677, t5861, t5875, t5878, t5880, t5900, t607, t61655, t61699, t61705, t61708, t61713, t61715, t61731, t61742, t61853, t62049, t62085, t62099, t62534, t62556, t62559, t62565, t62840, t62845, t67060, t68477, t68513, t68525, t68569, t69966, t70086, t70100, t70106, t70122, t70151, t70189, t70211, t70214, t70227, t70268, t70273, t70296, t70321, t70335, t70339, t70346, t70351, t70363, t70396, t70414, t70432, t70442, t70458, t70481, t70509, t70539, t70554, t70599, t70623, t70645, t70655, t70660, t70665, t70707, t70728, t70756, t70766, t70802, t70823, t70837, t70863, t70884, t70917, t70933, t973, t974, t998);
        let t70970 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2495::<F>(t1003, t1058, t1060, t11046, t11048, t14618, t14651, t18099, t18121, t18155, t21615, t21622, t21626, t3200, t4615, t4657, t4669, t4684, t4691, t50592, t5866, t5903, t5937, t5939, t5941, t70014);
        let t70985 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2496::<F>(t21689, t225, t21669, t10165, t1022, t1052, t1055, t1058, t1060, t1066, t11037, t11046, t11048, t11065, t11066, t14545, t14555, t14608, t14618, t14651, t1615, t1625, t1629, t1630, t17575, t17959, t18047, t18062, t18081, t18086, t18093, t18104, t18107, t18108, t18112, t18139, t18142, t18151, t18161, t21617, t21622, t21623, t21627, t21634, t21635, t21637, t21638, t21643, t21653, t21656, t21692, t3026, t3174, t3180, t3186, t3188, t3200, t353, t381, t383, t384, t388, t43515, t43516, t43562, t4552, t4649, t4660, t4665, t4669, t4673, t4674, t4677, t4680, t4681, t4684, t4685, t4689, t4693, t47841, t47857, t50465, t50516, t50610, t5914, t5919, t5920, t5928, t5929, t5932, t5933, t5936, t5943, t5944, t62994, t63004, t63183, t69871, t69942, t69966, t70009, t70012, t70014, t70068, t70081, t70082, t70086, t70938, t70970);
        let t71015 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2497::<F>(t21684, t225, t1066, t14529, t14555, t1635, t17575, t18061, t18062, t18074, t18166, t21663, t21692, t25757, t3169, t388, t4557, t4657, t4694, t50628, t5848, t5944, t61646, t63215);
        let t71049 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2498::<F>(t1052, t1065, t14529, t14552, t1603, t1634, t1635, t18047, t18074, t18165, t21614, t21676, t21677, t3026, t3169, t3174, t349, t388, t43604, t4665, t5920, t5944, t60971, t61061, t61621, t70938, t990);
        let t71055 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2499::<F>(t1070, t193, t336, t69335, t69337, t69340, t69343, t69346, t69350, t69353, t69357, t69469, t69471, t69860, t70985, t71015, t71049);
    t71055
}
