//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta669 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1977;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1978;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1979;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1980;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1981;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1982;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1983;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1984;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1985;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1986;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1987;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta669<F: Float>(t225: F, t29071: F, t1510: F, t24269: F, t2617: F, t26598: F, t28997: F, t29052: F, t4166: F, t5612: F, t5617: F, t812: F, t87068: F, t92491: F, t92546: F, t98325: F, t98328: F, t98330: F, t98334: F, t98339: F, t98342: F, t98345: F, t98349: F, t98353: F, t87073: F, t87078: F, t87080: F, t92502: F, t98356: F, t98359: F, t98363: F, t98367: F, t98374: F, t98380: F, t98384: F, t98387: F, t98392: F, t98396: F, t98399: F, t98402: F, t98405: F, t101499: F, t16673: F, t226: F, t235: F, t26661: F, t29000: F, t29041: F, t4234: F, t5585: F, t7102: F, t808: F, t81600: F, t84851: F, t84962: F, t87119: F, t87127: F, t87140: F, t98416: F, t98420: F, t98425: F, t98428: F, t98432: F, t98435: F, t87155: F, t92515: F, t92530: F, t98439: F, t98443: F, t98446: F, t98461: F, t98464: F, t98467: F, t98471: F, t98475: F, t98478: F, t98482: F, t98486: F, t98488: F, t98490: F, t98502: F, t16753: F, t26662: F, t5575: F, t7101: F, t7104: F, t87167: F, t87177: F, t92551: F, t92556: F, t92560: F, t92561: F, t92564: F, t92565: F, t98505: F, t98513: F, t98516: F, t98520: F, t98530: F, t98534: F, t29040: F, t814: F, t1509: F, t7823: F, t1499: F, t16805: F, t2051: F, t26654: F, t4162: F, t4291: F, t7839: F, t829: F, t84995: F, t87559: F, t92729: F, t92738: F, t92739: F, t92749: F, t92754: F, t98546: F, t98549: F, t98553: F, t98564: F, t98571: F, t2047: F, t5611: F, t5584: F, t13176: F, t16935: F, t26608: F, t26656: F, t29010: F, t4182: F, t4281: F, t7837: F, t85003: F, t87635: F, t87653: F, t87666: F, t92760: F, t92768: F, t92795: F, t98575: F, t13397: F, t16816: F, t16830: F, t17034: F, t26657: F, t26676: F, t82032: F, t85027: F, t87687: F, t87708: F, t87718: F, t92798: F, t92810: F, t92811: F, t92822: F, t92825: F, t98601: F, t98608: F, t98881: F, t98884: F, t2054: F, t26690: F, t26700: F, t26703: F, t4147: F, t4268: F, t4273: F, t59519: F, t85129: F, t855: F, t858: F, t866: F, t98941: F, t98945: F, t98963: F, t98966: F, t13053: F, t17049: F, t17090: F, t2053: F, t24297: F, t24305: F, t2597: F, t2713: F, t2718: F, t29056: F, t29080: F, t5637: F, t5658: F, t7092: F, t7842: F, t92938: F, t99003: F, t99019: F, t13042: F, t16804: F, t259: F, t29055: F, t7830: F, t865: F, t87929: F, t92966: F, t92976: F, t99033: F, t99036: F, t10110: F, t101335: F, t101359: F, t101504: F, t101509: F, t101540: F, t101569: F, t13065: F, t1492: F, t1527: F, t1528: F, t17050: F, t17069: F, t17070: F, t17092: F, t25168: F, t26582: F, t26653: F, t26679: F, t26680: F, t26728: F, t26729: F, t29060: F, t29091: F, t4300: F, t4301: F, t5558: F, t5636: F, t5657: F, t58143: F, t59466: F, t59537: F, t7084: F, t7087: F, t7106: F, t7841: F, t798: F, t84820: F, t85060: F, t86903: F, t86911: F, t86916: F, t86955: F, t87806: F, t87807: F, t87847: F, t87898: F, t87915: F, t92426: F, t92431: F, t92434: F, t92439: F, t92874: F, t92910: F, t92939: F, t92955: F, t92960: F, t98189: F, t98192: F, t98196: F, t98199: F, t98234: F, t98237: F, t98248: F, t98291: F, t98305: F, t98932: F, t98975: F, t98983: F, t98986: F, t98993: F, t98995: F, t99022: F) -> F {
        let (t101593, t101618) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1977::<F>(t225, t29071, t1510, t24269, t2617, t26598, t28997, t29052, t4166, t5612, t5617, t812, t87068, t92491, t92546, t98325, t98328, t98330, t98334, t98339, t98342, t98345, t98349, t98353);
        let t101634 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1978::<F>(t87073, t87078, t87080, t92502, t98356, t98359, t98363, t98367, t98374, t98380, t98384, t98387, t98392, t98396, t98399, t98402, t98405);
        let t101656 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1979::<F>(t101499, t16673, t226, t235, t2617, t26661, t29000, t29041, t4234, t5585, t7102, t808, t812, t81600, t84851, t84962, t87119, t87127, t87140, t98416, t98420, t98425, t98428, t98432, t98435);
        let t101672 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1980::<F>(t87155, t92515, t92530, t98439, t98443, t98446, t98461, t98464, t98467, t98471, t98475, t98478, t98482, t98486, t98488, t98490, t98502);
        let t101687 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1981::<F>(t16753, t26662, t4166, t5575, t7101, t7104, t812, t87167, t87177, t92551, t92556, t92560, t92561, t92564, t92565, t98505, t98513, t98516, t98520, t98530, t98534);
        let (t101698, t101705) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1982::<F>(t29040, t814, t1509, t7823, t1499, t16805, t2051, t26654, t4162, t4291, t7839, t812, t829, t84995, t87559, t92729, t92738, t92739, t92749, t92754, t98546, t98549, t98553, t98564, t98571);
        let (t101715, t101734) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1983::<F>(t2047, t5611, t5584, t101698, t13176, t16935, t2617, t26608, t26656, t29010, t4166, t4182, t4234, t4281, t4291, t7837, t829, t85003, t87635, t87653, t87666, t92760, t92768, t92795, t98575);
        let t101751 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1984::<F>(t101715, t13397, t16816, t16830, t17034, t26657, t26676, t4182, t4281, t82032, t85027, t87687, t87708, t87718, t92798, t92810, t92811, t92822, t92825, t98601, t98608, t98881, t98884);
        let t101761 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1985::<F>(t101593, t101618, t101634, t101656, t101672, t101687, t101705, t101734, t101751, t2054, t26690, t26700, t26703, t4147, t4268, t4273, t59519, t85129, t855, t858, t866, t98941, t98945, t98963, t98966);
        let t101797 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1986::<F>(t13053, t17049, t17090, t2053, t24297, t24305, t2597, t2713, t2718, t29056, t29080, t5637, t5658, t7092, t7842, t855, t92938, t99003, t99019);
        let t101828 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1987::<F>(t13042, t13053, t16804, t2047, t259, t2597, t2713, t2718, t29055, t29056, t29080, t7830, t7842, t855, t865, t87929, t92966, t92976, t99033, t99036);
        let t101832 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1988::<F>(t10110, t101335, t101359, t101504, t101509, t101540, t101569, t101761, t101797, t101828, t13065, t1492, t1527, t1528, t17050, t17069, t17070, t17092, t2054, t25168, t259, t2597, t26582, t26653, t26679, t26680, t26690, t26700, t26728, t26729, t2713, t2718, t29040, t29060, t29091, t4147, t4268, t4300, t4301, t5558, t5636, t5657, t58143, t59466, t59537, t7084, t7087, t7092, t7106, t7841, t7842, t798, t84820, t85060, t855, t866, t86903, t86911, t86916, t86955, t87806, t87807, t87847, t87898, t87915, t92426, t92431, t92434, t92439, t92874, t92910, t92939, t92955, t92960, t98189, t98192, t98196, t98199, t98234, t98237, t98248, t98291, t98305, t98932, t98975, t98983, t98986, t98993, t98995, t99022);
    t101832
}
