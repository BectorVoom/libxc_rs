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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta632<F: Float>(t87709: F, t87714: F, t87718: F, t87729: F, t87733: F, t13453: F, t2613: F, t26656: F, t26657: F, t2679: F, t4162: F, t4182: F, t4281: F, t4291: F, t7104: F, t7839: F, t82028: F, t82032: F, t82039: F, t85027: F, t87692: F, t87699: F, t87705: F, t87726: F, t92552: F, t2054: F, t24297: F, t26690: F, t2713: F, t4301: F, t46508: F, t82143: F, t82145: F, t82147: F, t82150: F, t855: F, t858: F, t87033: F, t87039: F, t92486: F, t92506: F, t92528: F, t92558: F, t92732: F, t92759: F, t92782: F, t92803: F, t87753: F, t225: F, t26732: F, t87776: F, t87779: F, t87786: F, t10110: F, t2597: F, t26582: F, t2719: F, t7830: F, t7841: F, t7842: F, t82172: F, t82174: F, t82182: F, t85101: F, t866: F, t87047: F, t87050: F, t87746: F, t87765: F, t87773: F, t87784: F, t9593: F, t87796: F, t87804: F, t13071: F, t13460: F, t2053: F, t24305: F, t24330: F, t25168: F, t26700: F, t26703: F, t26713: F, t26728: F, t2718: F, t2720: F, t4268: F, t4273: F, t46452: F, t82230: F, t82236: F, t87822: F, t87835: F, t87873: F, t26734: F, t13072: F, t13463: F, t1528: F, t218: F, t259: F, t47585: F, t7087: F, t7107: F, t85146: F, t85152: F, t87893: F, t92722: F, t87898: F, t87901: F, t87910: F, t87915: F, t87927: F, t87931: F, t10109: F, t7106: F, t13058: F, t13461: F, t4272: F, t4300: F, t82294: F, t82296: F, t85079: F, t87924: F, t10049: F, t13042: F, t13050: F, t13053: F, t13059: F, t13065: F, t1492: F, t1527: F, t24234: F, t24281: F, t24282: F, t24314: F, t24325: F, t2591: F, t26653: F, t26680: F, t26729: F, t2742: F, t4147: F, t47568: F, t47618: F, t7092: F, t7823: F, t798: F, t82076: F, t82099: F, t82131: F, t82135: F, t82209: F, t82211: F, t82221: F, t82259: F, t84820: F, t85129: F, t86909: F, t86923: F, t86961: F, t86972: F, t87010: F, t87013: F, t87755: F, t87861: F, t87866: F, t87904: F, t87907: F, t87920: F, t92400: F, t92402: F, t92406: F, t92428: F, t92431: F, t92432: F, t92434: F, t92439: F, t92464: F, t9590: F, t870: F, t10143: F, t7844: F, t1877: F, t2057: F, t22964: F, t23296: F, t24191: F, t25: F, t2522: F, t25385: F, t26563: F, t26740: F, t26756: F, t6542: F, t7110: F, t7114: F, t7845: F, t86718: F, t86722: F, t86798: F, t86821: F, t87984: F, t87998: F, t92356: F, t92359: F, t92362: F, t92364: F, t12971: F, t13471: F, t13487: F, t16596: F, t193: F, t202: F, t24339: F, t24344: F, t25365: F, t26744: F, t4119: F, t4255: F, t4303: F, t4314: F, t47645: F, t57912: F, t57921: F, t59580: F, t776: F, t7856: F, t86706: F, t89733: F, t13191: F, t13196: F, t1484: F, t1530: F, t2379: F, t24335: F, t25374: F, t2553: F, t2745: F, t2749: F, t57893: F, t58009: F, t58071: F, t84766: F, t84791: F, t84800: F, t86713: F, t86717: F, t868: F, t86815: F, t92276: F, t265: F, t394: F, t12606: F, t1409: F, t2064: F, t2250: F, t24380: F, t26807: F, t3966: F, t40: F, t607: F, t7131: F, t7865: F, t92270: F, t92309: F, t92349: F, dens_threshold: F, rho0: F, zeta_threshold: F, t23792: F, t23807: F, t25892: F, t25898: F, t25928: F, t25938: F, t25945: F, t28: F, t84797: F, t89843: F, t89881: F, t89928: F, t89972: F, t89987: F, t92271: F, t92295: F, t92299: F) -> (F, F, F) {
        let t92826 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1989::<F>(t87709, t87714, t87718, t87729, t87733, t13453, t2613, t26656, t26657, t2679, t4162, t4182, t4281, t4291, t7104, t7839, t82028, t82032, t82039, t85027, t87692, t87699, t87705, t87726, t92552);
        let t92839 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1990::<F>(t2054, t24297, t26690, t2713, t4301, t46508, t82143, t82145, t82147, t82150, t855, t858, t87033, t87039, t92486, t92506, t92528, t92558, t92732, t92759, t92782, t92803, t92826);
        let t92871 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1991::<F>(t87753, t225, t26732, t87776, t87779, t87786, t10110, t2597, t26582, t26690, t2719, t7830, t7841, t7842, t82172, t82174, t82182, t85101, t855, t866, t87047, t87050, t87746, t87765, t87773, t87784, t9593);
        let (t92872, t92874, t92907) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1992::<F>(t87796, t87804, t13071, t13460, t2053, t2054, t24305, t24330, t25168, t2597, t26700, t26703, t26713, t26728, t2718, t2720, t4268, t4273, t46452, t82230, t82236, t855, t87822);
        let (t92910, t92950) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1993::<F>(t87835, t87873, t225, t26734, t13072, t13463, t1528, t2054, t218, t259, t26582, t26703, t2713, t47585, t7087, t7107, t85146, t85152, t866, t87893, t92722);
        let (t92954, t92955, t92960, t92961, t92985) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1994::<F>(t87898, t87901, t87910, t87915, t87927, t87931, t10109, t7106, t13058, t13461, t1528, t24305, t25168, t26728, t2718, t4272, t4300, t4301, t7087, t82294, t82296, t85079, t855, t87924);
        let t92989 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1995::<F>(t10049, t13042, t13050, t13053, t13059, t13065, t13463, t1492, t1527, t2054, t24234, t24281, t24282, t24297, t24314, t24325, t259, t2591, t2597, t26653, t26680, t26729, t2713, t2718, t2742, t4147, t4268, t4273, t47568, t47618, t7087, t7092, t7107, t7823, t7830, t7841, t798, t82076, t82099, t82131, t82135, t82209, t82211, t82221, t82259, t84820, t85129, t855, t866, t86909, t86923, t86961, t86972, t87010, t87013, t87755, t87861, t87866, t87904, t87907, t87920, t92400, t92402, t92406, t92428, t92431, t92432, t92434, t92439, t92464, t92839, t92871, t92872, t92874, t92907, t92910, t92950, t92954, t92955, t92960, t92961, t92985, t9590);
        let (t92990, t93000, t93005) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1996::<F>(t870, t92989, t10143, t7844, t1877, t2057, t22964, t23296, t24191, t25, t2522, t25385, t26563, t26740, t26756, t6542, t7110, t7114, t7845, t86718, t86722, t86798, t86821, t87984, t87998, t92356, t92359, t92362, t92364);
        let t93052 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1997::<F>(t12971, t13471, t13487, t16596, t1877, t193, t202, t2057, t24191, t24339, t24344, t2522, t25365, t26563, t26740, t26744, t4119, t4255, t4303, t4314, t47645, t57912, t57921, t59580, t7110, t7114, t776, t7856, t86706, t870, t89733, t92989);
        let t93099 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1998::<F>(t13191, t13196, t1484, t1530, t1877, t2057, t2379, t24335, t24344, t2522, t25374, t2553, t26744, t2745, t2749, t4314, t57893, t58009, t58071, t7114, t7845, t84766, t84791, t84800, t86713, t86717, t868, t86815, t92276, t93000);
        let (t93100, t93113) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1999::<F>(t25, t265, t394, t93052, t93099, t12606, t1409, t2064, t2250, t24380, t26807, t3966, t40, t607, t7131, t7865, t92270, t92309, t92349, t93005, dens_threshold, rho0, zeta_threshold);
        let t93144 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2000::<F>(t1877, t2057, t23792, t23807, t24191, t24339, t2522, t25892, t25898, t25928, t25938, t25945, t26563, t28, t7110, t7845, t84797, t89843, t89881, t89928, t89972, t89987, t92271, t92295, t92299, t92990, t93000);
    (t93100, t93113, t93144)
}
