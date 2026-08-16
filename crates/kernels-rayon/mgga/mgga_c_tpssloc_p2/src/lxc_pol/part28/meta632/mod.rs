//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta632 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1990;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1991;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1992;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1993;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1994;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1995;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1996;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1997;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1998;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1999;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta632(t87709: f64, t87714: f64, t87718: f64, t87729: f64, t87733: f64, t13453: f64, t2613: f64, t26656: f64, t26657: f64, t2679: f64, t4162: f64, t4182: f64, t4281: f64, t4291: f64, t7104: f64, t7839: f64, t82028: f64, t82032: f64, t82039: f64, t85027: f64, t87692: f64, t87699: f64, t87705: f64, t87726: f64, t92552: f64, t2054: f64, t24297: f64, t26690: f64, t2713: f64, t4301: f64, t46508: f64, t82143: f64, t82145: f64, t82147: f64, t82150: f64, t855: f64, t858: f64, t87033: f64, t87039: f64, t92486: f64, t92506: f64, t92528: f64, t92558: f64, t92732: f64, t92759: f64, t92782: f64, t92803: f64, t87753: f64, t225: f64, t26732: f64, t87776: f64, t87779: f64, t87786: f64, t10110: f64, t2597: f64, t26582: f64, t2719: f64, t7830: f64, t7841: f64, t7842: f64, t82172: f64, t82174: f64, t82182: f64, t85101: f64, t866: f64, t87047: f64, t87050: f64, t87746: f64, t87765: f64, t87773: f64, t87784: f64, t9593: f64, t87796: f64, t87804: f64, t13071: f64, t13460: f64, t2053: f64, t24305: f64, t24330: f64, t25168: f64, t26700: f64, t26703: f64, t26713: f64, t26728: f64, t2718: f64, t2720: f64, t4268: f64, t4273: f64, t46452: f64, t82230: f64, t82236: f64, t87822: f64, t87835: f64, t87873: f64, t26734: f64, t13072: f64, t13463: f64, t1528: f64, t218: f64, t259: f64, t47585: f64, t7087: f64, t7107: f64, t85146: f64, t85152: f64, t87893: f64, t92722: f64, t87898: f64, t87901: f64, t87910: f64, t87915: f64, t87927: f64, t87931: f64, t10109: f64, t7106: f64, t13058: f64, t13461: f64, t4272: f64, t4300: f64, t82294: f64, t82296: f64, t85079: f64, t87924: f64, t10049: f64, t13042: f64, t13050: f64, t13053: f64, t13059: f64, t13065: f64, t1492: f64, t1527: f64, t24234: f64, t24281: f64, t24282: f64, t24314: f64, t24325: f64, t2591: f64, t26653: f64, t26680: f64, t26729: f64, t2742: f64, t4147: f64, t47568: f64, t47618: f64, t7092: f64, t7823: f64, t798: f64, t82076: f64, t82099: f64, t82131: f64, t82135: f64, t82209: f64, t82211: f64, t82221: f64, t82259: f64, t84820: f64, t85129: f64, t86909: f64, t86923: f64, t86961: f64, t86972: f64, t87010: f64, t87013: f64, t87755: f64, t87861: f64, t87866: f64, t87904: f64, t87907: f64, t87920: f64, t92400: f64, t92402: f64, t92406: f64, t92428: f64, t92431: f64, t92432: f64, t92434: f64, t92439: f64, t92464: f64, t9590: f64, t870: f64, t10143: f64, t7844: f64, t1877: f64, t2057: f64, t22964: f64, t23296: f64, t24191: f64, t25: f64, t2522: f64, t25385: f64, t26563: f64, t26740: f64, t26756: f64, t6542: f64, t7110: f64, t7114: f64, t7845: f64, t86718: f64, t86722: f64, t86798: f64, t86821: f64, t87984: f64, t87998: f64, t92356: f64, t92359: f64, t92362: f64, t92364: f64, t12971: f64, t13471: f64, t13487: f64, t16596: f64, t193: f64, t202: f64, t24339: f64, t24344: f64, t25365: f64, t26744: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t47645: f64, t57912: f64, t57921: f64, t59580: f64, t776: f64, t7856: f64, t86706: f64, t89733: f64, t13191: f64, t13196: f64, t1484: f64, t1530: f64, t2379: f64, t24335: f64, t25374: f64, t2553: f64, t2745: f64, t2749: f64, t57893: f64, t58009: f64, t58071: f64, t84766: f64, t84791: f64, t84800: f64, t86713: f64, t86717: f64, t868: f64, t86815: f64, t92276: f64, t265: f64, t394: f64, t12606: f64, t1409: f64, t2064: f64, t2250: f64, t24380: f64, t26807: f64, t3966: f64, t40: f64, t607: f64, t7131: f64, t7865: f64, t92270: f64, t92309: f64, t92349: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t23792: f64, t23807: f64, t25892: f64, t25898: f64, t25928: f64, t25938: f64, t25945: f64, t28: f64, t84797: f64, t89843: f64, t89881: f64, t89928: f64, t89972: f64, t89987: f64, t92271: f64, t92295: f64, t92299: f64) -> (f64, f64, f64) {
        let t92826 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1989(t87709, t87714, t87718, t87729, t87733, t13453, t2613, t26656, t26657, t2679, t4162, t4182, t4281, t4291, t7104, t7839, t82028, t82032, t82039, t85027, t87692, t87699, t87705, t87726, t92552);
        let t92839 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1990(t2054, t24297, t26690, t2713, t4301, t46508, t82143, t82145, t82147, t82150, t855, t858, t87033, t87039, t92486, t92506, t92528, t92558, t92732, t92759, t92782, t92803, t92826);
        let t92871 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1991(t87753, t225, t26732, t87776, t87779, t87786, t10110, t2597, t26582, t26690, t2719, t7830, t7841, t7842, t82172, t82174, t82182, t85101, t855, t866, t87047, t87050, t87746, t87765, t87773, t87784, t9593);
        let (t92872, t92874, t92907) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1992(t87796, t87804, t13071, t13460, t2053, t2054, t24305, t24330, t25168, t2597, t26700, t26703, t26713, t26728, t2718, t2720, t4268, t4273, t46452, t82230, t82236, t855, t87822);
        let (t92910, t92950) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1993(t87835, t87873, t225, t26734, t13072, t13463, t1528, t2054, t218, t259, t26582, t26703, t2713, t47585, t7087, t7107, t85146, t85152, t866, t87893, t92722);
        let (t92954, t92955, t92960, t92961, t92985) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1994(t87898, t87901, t87910, t87915, t87927, t87931, t10109, t7106, t13058, t13461, t1528, t24305, t25168, t26728, t2718, t4272, t4300, t4301, t7087, t82294, t82296, t85079, t855, t87924);
        let t92989 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1995(t10049, t13042, t13050, t13053, t13059, t13065, t13463, t1492, t1527, t2054, t24234, t24281, t24282, t24297, t24314, t24325, t259, t2591, t2597, t26653, t26680, t26729, t2713, t2718, t2742, t4147, t4268, t4273, t47568, t47618, t7087, t7092, t7107, t7823, t7830, t7841, t798, t82076, t82099, t82131, t82135, t82209, t82211, t82221, t82259, t84820, t85129, t855, t866, t86909, t86923, t86961, t86972, t87010, t87013, t87755, t87861, t87866, t87904, t87907, t87920, t92400, t92402, t92406, t92428, t92431, t92432, t92434, t92439, t92464, t92839, t92871, t92872, t92874, t92907, t92910, t92950, t92954, t92955, t92960, t92961, t92985, t9590);
        let (t92990, t93000, t93005) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1996(t870, t92989, t10143, t7844, t1877, t2057, t22964, t23296, t24191, t25, t2522, t25385, t26563, t26740, t26756, t6542, t7110, t7114, t7845, t86718, t86722, t86798, t86821, t87984, t87998, t92356, t92359, t92362, t92364);
        let t93052 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1997(t12971, t13471, t13487, t16596, t1877, t193, t202, t2057, t24191, t24339, t24344, t2522, t25365, t26563, t26740, t26744, t4119, t4255, t4303, t4314, t47645, t57912, t57921, t59580, t7110, t7114, t776, t7856, t86706, t870, t89733, t92989);
        let t93099 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1998(t13191, t13196, t1484, t1530, t1877, t2057, t2379, t24335, t24344, t2522, t25374, t2553, t26744, t2745, t2749, t4314, t57893, t58009, t58071, t7114, t7845, t84766, t84791, t84800, t86713, t86717, t868, t86815, t92276, t93000);
        let (t93100, t93113) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1999(t25, t265, t394, t93052, t93099, t12606, t1409, t2064, t2250, t24380, t26807, t3966, t40, t607, t7131, t7865, t92270, t92309, t92349, t93005, dens_threshold, rho0, zeta_threshold);
        let t93144 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2000(t1877, t2057, t23792, t23807, t24191, t24339, t2522, t25892, t25898, t25928, t25938, t25945, t26563, t28, t7110, t7845, t84797, t89843, t89881, t89928, t89972, t89987, t92271, t92295, t92299, t92990, t93000);
    (t93100, t93113, t93144)
}
