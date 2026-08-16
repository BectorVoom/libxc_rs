//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta726 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2349;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta726(t225: f64, t29665: f64, t8006: f64, t94490: f64, t11606: f64, t1190: f64, t1238: f64, t1252: f64, t15797: f64, t15820: f64, t1716: f64, t1720: f64, t19208: f64, t19213: f64, t19219: f64, t24615: f64, t27721: f64, t27784: f64, t27785: f64, t29536: f64, t29664: f64, t3593: f64, t498: f64, t6243: f64, t7283: f64, t7300: f64, t7301: f64, t7391: f64, t8014: f64, t8061: f64, t8088: f64, t86501: f64, t94391: f64, t94558: f64, t95912: f64, t29827: f64, t3640: f64, t103164: f64, t103213: f64, t103258: f64, t103279: f64, t103303: f64, t103341: f64, t103377: f64, t103415: f64, t103457: f64, t103488: f64, t104508: f64, t104534: f64, t104564: f64, t104596: f64, t104631: f64, t1254: f64, t1256: f64, t1763: f64, t19262: f64, t193: f64, t24905: f64, t24909: f64, t27838: f64, t27843: f64, t336: f64, t4700: f64, t5091: f64, t6270: f64, t6274: f64, t7398: f64, t86517: f64, t86524: f64, t95921: f64, t95925: f64, t28: f64, t265: f64, t504: f64, t100624: f64, t100805: f64, t1409: f64, t16558: f64, t2161: f64, t27850: f64, t29840: f64, t3966: f64, t52: f64, t5398: f64, t607: f64, t7402: f64, t8097: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t103125: f64, t113: f64, t1393: f64, t1849: f64, t19289: f64, t19450: f64, t20098: f64, t2114: f64, t2165: f64, t2167: f64, t27903: f64, t29497: f64, t33690: f64, t4073: f64, t96355: f64, t96358: f64, t96360: f64, t96738: f64, t96740: f64, t96746: f64, t96755: f64, t96758: f64, t96760: f64, t96763: f64, t96765: f64, t96767: f64, t5456: f64, t7263: f64, t2109: f64, t96461: f64, t96469: f64, t96425: f64, t22549: f64, t24514: f64, t24517: f64, t26016: f64, t27298: f64, t83717: f64, t85501: f64, t90098: f64, t90101: f64, t90104: f64, t96135: f64, t96138: f64, t96418: f64, t96422: f64, t96466: f64, t96473: f64, t2110: f64, t26009: f64, t27937: f64, t27979: f64, t7256: f64, t7259: f64, t90114: f64, t96102: f64, t96110: f64, t96115: f64, t96120: f64, t96383: f64, t96443: f64, t96646: f64, t26012: f64, t7974: f64, t1860: f64, t26024: f64, t26028: f64, t27303: f64, t27308: f64, t27365: f64, t27956: f64, t29481: f64, t6486: f64, t7255: f64, t7428: f64, t7975: f64, t7978: f64, t96045: f64, t96379: f64, t96458: f64, t5415: f64, t55: f64, t17635: f64, t17686: f64, t17691: f64, t1864: f64, t24498: f64, t26090: f64, t27311: f64, t27332: f64, t27356: f64, t27364: f64, t29474: f64, t29475: f64, t29478: f64, t3961: f64, t6495: f64, t6509: f64, t67: f64, t7246: f64, t7251: f64, t7432: f64, t7445: f64, t83803: f64, t85539: f64, t96025: f64, t96157: f64, t96393: f64, t26063: f64, t26067: f64, t27341: f64, t27966: f64, t95981: f64, t96028: f64, t96072: f64, t96406: f64, t96479: f64, t96482: f64, t2108: f64, t2240: f64, t5392: f64, t605: f64, t1410: f64, t24520: f64, t24526: f64, t27972: f64, t27976: f64, t6492: f64, t9239: f64, t96502: f64, t96506: f64, t96517: f64, t96521: f64, t96553: f64, t96556: f64, t26070: f64, t26073: f64, t26076: f64, t27961: f64, t27982: f64, t7435: f64, t85480: f64, t85536: f64, t96403: f64, t96559: f64, t96562: f64, t55921: f64, t7245: f64, t12571: f64, t27331: f64, t29473: f64, t33: f64, t26055: f64, t96535: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t104669 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341(t225, t29665, t8006, t94490, t11606, t1190, t1238, t1252, t15797, t15820, t1716, t1720, t19208, t19213, t19219, t24615, t27721, t27784, t27785, t29536, t29664, t3593, t498, t6243, t7283, t7300, t7301, t7391, t8014, t8061, t8088, t86501, t94391, t94558, t95912);
        let t104708 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342(t29827, t3640, t103164, t103213, t103258, t103279, t103303, t103341, t103377, t103415, t103457, t103488, t104508, t104534, t104564, t104596, t104631, t104669, t1254, t1256, t1763, t19262, t193, t24905, t24909, t27838, t27843, t336, t4700, t5091, t6270, t6274, t7398, t86517, t86524, t95921, t95925);
        let t104721 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2343(t28, t265, t504, t100624, t104708, t100805, t1409, t16558, t2161, t27850, t29840, t3966, t52, t5398, t607, t7402, t8097, dens_threshold, rho1, zeta_threshold);
        let t104727 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344(t103125, t104721, t113, t1393, t1849, t19289, t19450, t20098, t2114, t2165, t2167, t27903, t29497, t33690, t4073, t96355, t96358, t96360, t96738, t96740, t96746, t96755, t96758, t96760, t96763, t96765, t96767);
        let (t104729, t104758) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2345(t5456, t7263, t2109, t96461, t96469, t96425, t22549, t24514, t24517, t26016, t27298, t83717, t85501, t90098, t90101, t90104, t96135, t96138, t96418, t96422, t96466, t96473);
        let t104783 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346(t2110, t24517, t26009, t26016, t27298, t27937, t27979, t7256, t7259, t90114, t96102, t96110, t96115, t96120, t96383, t96443, t96646);
        let t104813 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347(t26012, t7974, t1860, t2109, t22549, t24514, t26009, t26024, t26028, t27303, t27308, t27365, t27956, t29481, t6486, t7255, t7428, t7975, t7978, t96045, t96379, t96458);
        let t104858 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348(t5415, t55, t16558, t17635, t17686, t17691, t1860, t1864, t24498, t26090, t27311, t27332, t27356, t27364, t29474, t29475, t29478, t29481, t3961, t3966, t607, t6486, t6495, t6509, t67, t7246, t7251, t7428, t7432, t7445, t83803, t85539, t96025, t96157, t96393);
        let t104885 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2349(t2110, t26063, t26067, t27332, t27341, t27966, t7256, t7259, t7432, t95981, t96028, t96072, t96406, t96479, t96482);
        let t104916 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350(t2108, t2240, t5392, t1409, t605, t1410, t2110, t24520, t24526, t26009, t27972, t27976, t6492, t7246, t9239, t96502, t96506, t96517, t96521, t96553, t96556);
        let t104942 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351(t2110, t24514, t26070, t26073, t26076, t27303, t27365, t27961, t27982, t7256, t7259, t7435, t7975, t85480, t85536, t96403, t96559, t96562);
        let t104971 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352(t55921, t7245, t12571, t27331, t2240, t29473, t33, t2110, t26055, t26070, t26073, t26076, t26090, t27308, t27311, t27341, t6492, t7435, t7975, t7978, t96535);
    (t104727, t104729, t104758, t104783, t104813, t104858, t104885, t104916, t104942, t104971)
}
