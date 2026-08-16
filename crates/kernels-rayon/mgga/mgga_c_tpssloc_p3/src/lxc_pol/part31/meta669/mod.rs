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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta669(t225: f64, t29071: f64, t1510: f64, t24269: f64, t2617: f64, t26598: f64, t28997: f64, t29052: f64, t4166: f64, t5612: f64, t5617: f64, t812: f64, t87068: f64, t92491: f64, t92546: f64, t98325: f64, t98328: f64, t98330: f64, t98334: f64, t98339: f64, t98342: f64, t98345: f64, t98349: f64, t98353: f64, t87073: f64, t87078: f64, t87080: f64, t92502: f64, t98356: f64, t98359: f64, t98363: f64, t98367: f64, t98374: f64, t98380: f64, t98384: f64, t98387: f64, t98392: f64, t98396: f64, t98399: f64, t98402: f64, t98405: f64, t101499: f64, t16673: f64, t226: f64, t235: f64, t26661: f64, t29000: f64, t29041: f64, t4234: f64, t5585: f64, t7102: f64, t808: f64, t81600: f64, t84851: f64, t84962: f64, t87119: f64, t87127: f64, t87140: f64, t98416: f64, t98420: f64, t98425: f64, t98428: f64, t98432: f64, t98435: f64, t87155: f64, t92515: f64, t92530: f64, t98439: f64, t98443: f64, t98446: f64, t98461: f64, t98464: f64, t98467: f64, t98471: f64, t98475: f64, t98478: f64, t98482: f64, t98486: f64, t98488: f64, t98490: f64, t98502: f64, t16753: f64, t26662: f64, t5575: f64, t7101: f64, t7104: f64, t87167: f64, t87177: f64, t92551: f64, t92556: f64, t92560: f64, t92561: f64, t92564: f64, t92565: f64, t98505: f64, t98513: f64, t98516: f64, t98520: f64, t98530: f64, t98534: f64, t29040: f64, t814: f64, t1509: f64, t7823: f64, t1499: f64, t16805: f64, t2051: f64, t26654: f64, t4162: f64, t4291: f64, t7839: f64, t829: f64, t84995: f64, t87559: f64, t92729: f64, t92738: f64, t92739: f64, t92749: f64, t92754: f64, t98546: f64, t98549: f64, t98553: f64, t98564: f64, t98571: f64, t2047: f64, t5611: f64, t5584: f64, t13176: f64, t16935: f64, t26608: f64, t26656: f64, t29010: f64, t4182: f64, t4281: f64, t7837: f64, t85003: f64, t87635: f64, t87653: f64, t87666: f64, t92760: f64, t92768: f64, t92795: f64, t98575: f64, t13397: f64, t16816: f64, t16830: f64, t17034: f64, t26657: f64, t26676: f64, t82032: f64, t85027: f64, t87687: f64, t87708: f64, t87718: f64, t92798: f64, t92810: f64, t92811: f64, t92822: f64, t92825: f64, t98601: f64, t98608: f64, t98881: f64, t98884: f64, t2054: f64, t26690: f64, t26700: f64, t26703: f64, t4147: f64, t4268: f64, t4273: f64, t59519: f64, t85129: f64, t855: f64, t858: f64, t866: f64, t98941: f64, t98945: f64, t98963: f64, t98966: f64, t13053: f64, t17049: f64, t17090: f64, t2053: f64, t24297: f64, t24305: f64, t2597: f64, t2713: f64, t2718: f64, t29056: f64, t29080: f64, t5637: f64, t5658: f64, t7092: f64, t7842: f64, t92938: f64, t99003: f64, t99019: f64, t13042: f64, t16804: f64, t259: f64, t29055: f64, t7830: f64, t865: f64, t87929: f64, t92966: f64, t92976: f64, t99033: f64, t99036: f64, t10110: f64, t101335: f64, t101359: f64, t101504: f64, t101509: f64, t101540: f64, t101569: f64, t13065: f64, t1492: f64, t1527: f64, t1528: f64, t17050: f64, t17069: f64, t17070: f64, t17092: f64, t25168: f64, t26582: f64, t26653: f64, t26679: f64, t26680: f64, t26728: f64, t26729: f64, t29060: f64, t29091: f64, t4300: f64, t4301: f64, t5558: f64, t5636: f64, t5657: f64, t58143: f64, t59466: f64, t59537: f64, t7084: f64, t7087: f64, t7106: f64, t7841: f64, t798: f64, t84820: f64, t85060: f64, t86903: f64, t86911: f64, t86916: f64, t86955: f64, t87806: f64, t87807: f64, t87847: f64, t87898: f64, t87915: f64, t92426: f64, t92431: f64, t92434: f64, t92439: f64, t92874: f64, t92910: f64, t92939: f64, t92955: f64, t92960: f64, t98189: f64, t98192: f64, t98196: f64, t98199: f64, t98234: f64, t98237: f64, t98248: f64, t98291: f64, t98305: f64, t98932: f64, t98975: f64, t98983: f64, t98986: f64, t98993: f64, t98995: f64, t99022: f64) -> f64 {
        let (t101593, t101618) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1977(t225, t29071, t1510, t24269, t2617, t26598, t28997, t29052, t4166, t5612, t5617, t812, t87068, t92491, t92546, t98325, t98328, t98330, t98334, t98339, t98342, t98345, t98349, t98353);
        let t101634 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1978(t87073, t87078, t87080, t92502, t98356, t98359, t98363, t98367, t98374, t98380, t98384, t98387, t98392, t98396, t98399, t98402, t98405);
        let t101656 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1979(t101499, t16673, t226, t235, t2617, t26661, t29000, t29041, t4234, t5585, t7102, t808, t812, t81600, t84851, t84962, t87119, t87127, t87140, t98416, t98420, t98425, t98428, t98432, t98435);
        let t101672 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1980(t87155, t92515, t92530, t98439, t98443, t98446, t98461, t98464, t98467, t98471, t98475, t98478, t98482, t98486, t98488, t98490, t98502);
        let t101687 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1981(t16753, t26662, t4166, t5575, t7101, t7104, t812, t87167, t87177, t92551, t92556, t92560, t92561, t92564, t92565, t98505, t98513, t98516, t98520, t98530, t98534);
        let (t101698, t101705) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1982(t29040, t814, t1509, t7823, t1499, t16805, t2051, t26654, t4162, t4291, t7839, t812, t829, t84995, t87559, t92729, t92738, t92739, t92749, t92754, t98546, t98549, t98553, t98564, t98571);
        let (t101715, t101734) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1983(t2047, t5611, t5584, t101698, t13176, t16935, t2617, t26608, t26656, t29010, t4166, t4182, t4234, t4281, t4291, t7837, t829, t85003, t87635, t87653, t87666, t92760, t92768, t92795, t98575);
        let t101751 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1984(t101715, t13397, t16816, t16830, t17034, t26657, t26676, t4182, t4281, t82032, t85027, t87687, t87708, t87718, t92798, t92810, t92811, t92822, t92825, t98601, t98608, t98881, t98884);
        let t101761 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1985(t101593, t101618, t101634, t101656, t101672, t101687, t101705, t101734, t101751, t2054, t26690, t26700, t26703, t4147, t4268, t4273, t59519, t85129, t855, t858, t866, t98941, t98945, t98963, t98966);
        let t101797 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1986(t13053, t17049, t17090, t2053, t24297, t24305, t2597, t2713, t2718, t29056, t29080, t5637, t5658, t7092, t7842, t855, t92938, t99003, t99019);
        let t101828 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1987(t13042, t13053, t16804, t2047, t259, t2597, t2713, t2718, t29055, t29056, t29080, t7830, t7842, t855, t865, t87929, t92966, t92976, t99033, t99036);
        let t101832 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1988(t10110, t101335, t101359, t101504, t101509, t101540, t101569, t101761, t101797, t101828, t13065, t1492, t1527, t1528, t17050, t17069, t17070, t17092, t2054, t25168, t259, t2597, t26582, t26653, t26679, t26680, t26690, t26700, t26728, t26729, t2713, t2718, t29040, t29060, t29091, t4147, t4268, t4300, t4301, t5558, t5636, t5657, t58143, t59466, t59537, t7084, t7087, t7092, t7106, t7841, t7842, t798, t84820, t85060, t855, t866, t86903, t86911, t86916, t86955, t87806, t87807, t87847, t87898, t87915, t92426, t92431, t92434, t92439, t92874, t92910, t92939, t92955, t92960, t98189, t98192, t98196, t98199, t98234, t98237, t98248, t98291, t98305, t98932, t98975, t98983, t98986, t98993, t98995, t99022);
    t101832
}
