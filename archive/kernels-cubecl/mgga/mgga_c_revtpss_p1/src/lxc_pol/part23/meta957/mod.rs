//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta957 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3201;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3202;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3203;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3204;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3205;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3206;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3207;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3208;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3209;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3210;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3211;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta957<F: Float>(t12772: F, t24797: F, t3625: F, t21004: F, t21030: F, t21121: F, t57707: F, t57710: F, t59233: F, t59411: F, t71738: F, t71740: F, t71742: F, t71744: F, t71749: F, t71751: F, t59337: F, t59339: F, t71827: F, t71845: F, t71859: F, t71880: F, t71883: F, t71886: F, t71908: F, t71920: F, t71928: F, t1256: F, t24684: F, t24700: F, t1791: F, t21107: F, t5287: F, t70210: F, t71931: F, t71971: F, t71974: F, t71976: F, t72000: F, t72005: F, t72017: F, t1803: F, t20850: F, t1238: F, t1248: F, t12809: F, t12866: F, t13045: F, t17351: F, t17426: F, t17605: F, t17654: F, t17693: F, t17694: F, t17709: F, t1794: F, t20851: F, t21008: F, t24228: F, t24569: F, t3611: F, t3720: F, t44225: F, t5284: F, t5320: F, t5405: F, t5406: F, t6688: F, t70890: F, t71112: F, t83040: F, t83943: F, t83950: F, t44865: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t43888: F, t56236: F, t56343: F, t56345: F, t56360: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F, t1234: F, t24680: F, t1222: F, t140: F, t24826: F, t1235: F, t17283: F, t21085: F, t21236: F, t24636: F, t3667: F, t371: F, t372: F, t482: F, t5323: F, t5348: F, t5373: F, t59419: F, t59426: F, t6647: F, t71513: F, t72064: F, t72071: F, t82305: F, t82340: F, t82367: F, t82438: F, t82467: F, t82510: F, t82542: F, t82570: F, t82608: F, t82639: F, t82669: F, t82696: F, t82730: F, t82763: F, t82792: F, t82831: F, t82864: F, t82904: F, t82929: F, t82950: F, t82978: F, t83016: F, t83051: F, t83081: F, t83117: F, t83145: F, t83170: F, t83240: F, t83259: F, t83281: F, t83307: F, t83322: F, t83352: F, t83361: F, t83384: F, t83414: F, t83451: F, t83480: F, t83502: F, t83526: F, t83562: F, t83592: F, t83617: F, t83640: F, t83683: F, t83712: F, t83741: F, t83771: F, t83808: F, t83836: F, t83865: F, t83893: F, t83915: F, t83938: F, t83973: F, t83996: F, t84020: F, t84036: F, t84049: F, t12633: F, t1274: F, t1775: F, t17973: F, t17995: F, t18087: F, t1828: F, t20710: F, t20722: F, t20741: F, t20756: F, t20760: F, t21390: F, t21394: F, t21617: F, t21618: F, t21621: F, t225: F, t24515: F, t24892: F, t24900: F, t25016: F, t3556: F, t3572: F, t3732: F, t3736: F, t3737: F, t460: F, t494: F, t5220: F, t5246: F, t5251: F, t5417: F, t5422: F, t5428: F, t5429: F, t6573: F, t6745: F, t68658: F, t72808: F, t73051: F, t73055: F, t1210: F, t1211: F, t12628: F, t1277: F, t1294: F, t17974: F, t17986: F, t17987: F, t18037: F, t18059: F, t18114: F, t1829: F, t20704: F, t20709: F, t20714: F, t20727: F, t20740: F, t21348: F, t21389: F, t21624: F, t24509: F, t25015: F, t3561: F, t5245: F, t5414: F, t5423: F, t59464: F, t6564: F, t6574: F, t6580: F, t6588: F, t6744: F, t72787: F, t72794: F, t72959: F, t82525: F, t1209: F, t24864: F, t1215: F, t18054: F, t18062: F, t20697: F, t20700: F, t20753: F, t21382: F, t24519: F, t24524: F, t25022: F, t3567: F, t45438: F, t45552: F, t5225: F, t5237: F, t5497: F, t5498: F, t6587: F, t6703: F, t72877: F, t82514: F, t83551: F, t83567: F, t24739: F, t3153: F, t12717: F, t12744: F, t1287: F, t16756: F, t17307: F, t1774: F, t1822: F, t20703: F, t21443: F, t21513: F, t21518: F, t21524: F, t24713: F, t24977: F, t24994: F, t45859: F, t45863: F, t5463: F, t5465: F, t5480: F, t57264: F, t59674: F, t59788: F, t59817: F, t60037: F, t68674: F, t72397: F) -> (F, F, F, F, F, F) {
        let t84066 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3201::<F>(t12772, t24797, t3625, t21004, t21030, t21121, t57707, t57710, t59233, t59411, t71738, t71740, t71742, t71744, t71749, t71751);
        let (t84078, t84082) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3202::<F>(t59337, t59339, t71827, t71845, t71859, t71880, t71883, t71886, t71908, t71920, t71928, t1256, t24684);
        let t84094 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3203::<F>(t1256, t24700, t1791, t21107, t5287, t70210, t71931, t71971, t71974, t71976, t72000, t72005, t72017, t84082);
        let t84132 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3204::<F>(t1803, t20850, t1238, t1248, t12809, t12866, t13045, t17351, t17426, t17605, t17654, t17693, t17694, t17709, t1794, t20851, t21008, t24228, t24569, t3611, t3625, t3720, t44225, t5284, t5320, t5405, t5406, t6688, t70890, t71112, t83040, t83943, t83950);
        let t84156 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3205::<F>(t44865, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t84174 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3206::<F>(t43888, t56236, t56343, t56345, t56360, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let (t84175, t84197) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3207::<F>(t84156, t84174, t1234, t24680, t1222, t140, t24826, t1235, t1238, t17283, t21085, t21236, t24636, t3667, t371, t372, t482, t5323, t5348, t5373, t59419, t59426, t6647, t71513, t72064, t72071);
        let t84203 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3208::<F>(t82305, t82340, t82367, t82438, t82467, t82510, t82542, t82570, t82608, t82639, t82669, t82696, t82730, t82763, t82792, t82831, t82864, t82904, t82929, t82950, t82978, t83016, t83051, t83081, t83117, t83145, t83170, t83240, t83259, t83281, t83307, t83322, t83352, t83361, t83384, t83414, t83451, t83480, t83502, t83526, t83562, t83592, t83617, t83640, t83683, t83712, t83741, t83771, t83808, t83836, t83865, t83893, t83915, t83938, t83973, t83996, t84020, t84036, t84049, t84066, t84078, t84094, t84132, t84197);
        let t84241 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3209::<F>(t12633, t1274, t1775, t17973, t17995, t18087, t1828, t20710, t20722, t20741, t20756, t20760, t21390, t21394, t21617, t21618, t21621, t225, t24515, t24892, t24900, t25016, t3556, t3572, t3732, t3736, t3737, t460, t494, t5220, t5246, t5251, t5417, t5422, t5428, t5429, t6573, t6745, t68658, t72808, t73051, t73055, t84203);
        let t84290 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3210::<F>(t1210, t1211, t12628, t1274, t1277, t1294, t1775, t17973, t17974, t17986, t17987, t17995, t18037, t18059, t18114, t1829, t20704, t20709, t20714, t20727, t20740, t21348, t21389, t21390, t21621, t21624, t24509, t25015, t3561, t3737, t5245, t5251, t5414, t5417, t5423, t59464, t6564, t6574, t6580, t6588, t6744, t72787, t72794, t72959, t82525);
        let t84337 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3211::<F>(t1209, t24864, t1210, t1211, t1215, t1274, t1277, t1294, t1775, t18054, t18059, t18062, t20697, t20700, t20722, t20753, t20760, t21382, t24519, t24524, t25022, t3556, t3567, t3572, t45438, t45552, t5225, t5237, t5251, t5497, t5498, t6580, t6587, t6588, t6703, t72877, t82514, t83551, t83567);
        let t84392 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3212::<F>(t24739, t3153, t1234, t1248, t12717, t12744, t1287, t16756, t17307, t1774, t1822, t20703, t21443, t21513, t21518, t21524, t24713, t24977, t24994, t45859, t45863, t5463, t5465, t5480, t57264, t59674, t59788, t59817, t60037, t68674, t72397);
    (t84175, t84203, t84241, t84290, t84337, t84392)
}
