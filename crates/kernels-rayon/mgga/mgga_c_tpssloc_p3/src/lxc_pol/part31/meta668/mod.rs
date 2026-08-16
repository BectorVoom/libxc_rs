//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta668 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1966;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta668(t225: f64, t29095: f64, t26729: f64, t866: f64, t86930: f64, t86931: f64, t92415: f64, t92425: f64, t98202: f64, t98205: f64, t98213: f64, t98222: f64, t98227: f64, t98279: f64, t92578: f64, t98610: f64, t98612: f64, t98614: f64, t98616: f64, t98618: f64, t98620: f64, t98622: f64, t98624: f64, t98626: f64, t98629: f64, t98631: f64, t98633: f64, t98635: f64, t98637: f64, t98639: f64, t98642: f64, t84857: f64, t84859: f64, t87213: f64, t92580: f64, t92582: f64, t98647: f64, t98651: f64, t98655: f64, t98659: f64, t98663: f64, t98668: f64, t98672: f64, t98674: f64, t98676: f64, t98678: f64, t98680: f64, t98682: f64, t98685: f64, t81789: f64, t87237: f64, t87243: f64, t87268: f64, t92590: f64, t92599: f64, t92603: f64, t92607: f64, t92614: f64, t92615: f64, t98690: f64, t98694: f64, t98696: f64, t98701: f64, t98703: f64, t98707: f64, t98709: f64, t98711: f64, t84896: f64, t84897: f64, t87304: f64, t87306: f64, t92626: f64, t92627: f64, t92630: f64, t98715: f64, t98717: f64, t98719: f64, t98721: f64, t98723: f64, t98725: f64, t98728: f64, t98731: f64, t98733: f64, t98736: f64, t98738: f64, t87319: f64, t87320: f64, t92635: f64, t92645: f64, t98744: f64, t98746: f64, t98750: f64, t98752: f64, t98754: f64, t98758: f64, t98762: f64, t98766: f64, t98770: f64, t98774: f64, t98777: f64, t98782: f64, t98787: f64, t98791: f64, t81903: f64, t87335: f64, t87345: f64, t87387: f64, t92646: f64, t92647: f64, t92649: f64, t92650: f64, t92653: f64, t92657: f64, t92675: f64, t98796: f64, t98798: f64, t98801: f64, t98803: f64, t98808: f64, t98811: f64, t98814: f64, t87403: f64, t87405: f64, t87414: f64, t87425: f64, t87432: f64, t92679: f64, t98818: f64, t98820: f64, t98822: f64, t98824: f64, t98826: f64, t98828: f64, t98830: f64, t98833: f64, t98836: f64, t98838: f64, t98842: f64, t98844: f64, t84921: f64, t84932: f64, t87437: f64, t87438: f64, t87440: f64, t87445: f64, t92697: f64, t92705: f64, t92710: f64, t92713: f64, t98847: f64, t98849: f64, t98851: f64, t98853: f64, t98858: f64, t98862: f64, t98868: f64, t98871: f64, t1528: f64, t17056: f64, t218: f64, t25168: f64, t259: f64, t26728: f64, t2713: f64, t29091: f64, t86983: f64, t86991: f64, t86994: f64, t92386: f64, t98251: f64, t98256: f64, t98264: f64, t98277: f64, t29099: f64, t13463: f64, t17057: f64, t17063: f64, t17092: f64, t26582: f64, t4268: f64, t7087: f64, t7107: f64, t7830: f64, t87042: f64, t87050: f64, t92394: f64, t92486: f64, t98315: f64, t98319: f64, t98322: f64, t10109: f64, t7841: f64, t13065: f64, t17052: f64, t17090: f64, t2054: f64, t24305: f64, t26703: f64, t26713: f64, t4147: f64, t4272: f64, t4301: f64, t5658: f64, t59498: f64, t7092: f64, t7842: f64, t85101: f64, t87779: f64, t92846: f64, t92847: f64, t92862: f64, t92866: f64, t92872: f64, t98921: f64, t98923: f64, t98927: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t101359 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965(t225, t29095, t26729, t866, t86930, t86931, t92415, t92425, t98202, t98205, t98213, t98222, t98227, t98279);
        let t101398 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1966(t92578, t98610, t98612, t98614, t98616, t98618, t98620, t98622, t98624, t98626, t98629, t98631, t98633, t98635, t98637, t98639, t98642);
        let t101413 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967(t84857, t84859, t87213, t92580, t92582, t98647, t98651, t98655, t98659, t98663, t98668, t98672, t98674, t98676, t98678, t98680, t98682, t98685);
        let t101425 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1968(t81789, t87237, t87243, t87268, t92590, t92599, t92603, t92607, t92614, t92615, t98690, t98694, t98696, t98701, t98703, t98707, t98709, t98711);
        let t101439 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1969(t84896, t84897, t87304, t87306, t92626, t92627, t92630, t98715, t98717, t98719, t98721, t98723, t98725, t98728, t98731, t98733, t98736, t98738);
        let t101456 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1970(t87319, t87320, t92635, t92645, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766, t98770, t98774, t98777, t98782, t98787, t98791);
        let t101468 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1971(t81903, t87335, t87345, t87387, t92646, t92647, t92649, t92650, t92653, t92657, t92675, t98796, t98798, t98801, t98803, t98808, t98811, t98814);
        let t101486 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972(t87403, t87405, t87414, t87425, t87432, t92679, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98833, t98836, t98838, t98842, t98844);
        let t101496 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1973(t84921, t84932, t87437, t87438, t87440, t87445, t92697, t92705, t92710, t92713, t98847, t98849, t98851, t98853, t98858, t98862, t98868, t98871);
        let (t101499, t101504) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974(t101398, t101413, t101425, t101439, t101456, t101468, t101486, t101496, t1528, t17056, t218, t25168, t259, t26728, t2713, t29091, t86983, t86991, t86994, t92386, t98251, t98256, t98264, t98277);
        let (t101509, t101540) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1975(t225, t29099, t13463, t17057, t17063, t17092, t25168, t26582, t4268, t7087, t7107, t7830, t87042, t87050, t92394, t92486, t98315, t98319, t98322);
        let t101569 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976(t10109, t7841, t13065, t13463, t1528, t17052, t17090, t2054, t24305, t25168, t26703, t26713, t4147, t4272, t4301, t5658, t59498, t7092, t7107, t7830, t7842, t85101, t87779, t92846, t92847, t92862, t92866, t92872, t98921, t98923, t98927);
    (t101359, t101499, t101504, t101509, t101540, t101569)
}
