//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta598 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2027;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2028;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2029;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2030;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2031;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2032;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2033;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2034;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2035;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2036;
use chunk10::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2037;
use chunk11::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta598<F: Float>(t103431: F, t25375: F, t212: F, t28340: F, t689: F, t780: F, t103182: F, t93281: F, t103421: F, t7058: F, t25317: F, t25383: F, t26475: F, t28385: F, t28405: F, t28417: F, t28436: F, t7070: F, t7415: F, t7766: F, t8012: F, t886: F, t92917: F, t93126: F, t95930: F, t95937: F, t95945: F, t95948: F, t99303: F, t102954: F, t102977: F, t103008: F, t103033: F, t103065: F, t103100: F, t103137: F, t103166: F, t103210: F, t103242: F, t103380: F, t103412: F, t103451: F, t103488: F, t103519: F, t892: F, t26425: F, t98648: F, t1940: F, t2255: F, t7428: F, t102917: F, t2071: F, t2403: F, t25215: F, t26585: F, t27173: F, t27387: F, t28291: F, t28472: F, t30: F, t4541: F, t7432: F, t8020: F, t98652: F, t98675: F, t98705: F, t98709: F, t98736: F, t98780: F, t98793: F, t99543: F, t11064: F, t8019: F, t25446: F, t26581: F, t26590: F, t27376: F, t27391: F, t28456: F, t51780: F, t7010: F, t7749: F, t7991: F, t95511: F, t98627: F, t98659: F, t98662: F, t98740: F, t98743: F, t98751: F, t98755: F, t98768: F, t99550: F, t100858: F, t14749: F, t14767: F, t15071: F, t1544: F, t1583: F, t198: F, t207: F, t2394: F, t2832: F, t28460: F, t4343: F, t4433: F, t61155: F, t61182: F, t63186: F, t95527: F, t95964: F, t98759: F, t98786: F, t102854: F, t14365: F, t14468: F, t18875: F, t2408: F, t2430: F, t27375: F, t27384: F, t4537: F, t61102: F, t61203: F, t63164: F, t775: F, t8031: F, t890: F, t95976: F, t98651: F, t98779: F, t265: F, t393: F, t102867: F, t102905: F, t13312: F, t1469: F, t2078: F, t2258: F, t26626: F, t28523: F, t4186: F, t45: F, t606: F, t7449: F, t8040: F, dens_threshold: F, rho0: F, zeta_threshold: F, t100944: F, t100947: F, t100953: F, t100958: F, t100969: F, t100978: F, t101029: F, t101032: F, t101086: F, t102851: F, t102858: F, t1711: F, t27793: F, t27800: F, t3351: F, t100964: F, t100975: F, t100982: F, t101016: F, t101065: F, t101093: F, t102864: F, t102877: F, t102888: F, t25760: F, t25763: F, t25778: F, t27764: F, t27806: F, t27817: F, t7207: F, t100993: F, t100997: F, t101035: F, t101040: F, t101051: F, t101061: F, t101070: F, t101074: F, t1113: F, t25752: F, t25767: F, t27773: F, t33: F, t8046: F, t100988: F, t101012: F, t101055: F, t101083: F, t101099: F, t25781: F, t25784: F, t27770: F, t27777: F, t27802: F, t27810: F, t7200: F, t7862: F, t7869: F, t502: F, t2085: F, t26666: F, t28578: F, t57: F, t7468: F, t8059: F, rho1: F) -> (F, F) {
        let t103549 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2027::<F>(t103431, t25375, t212, t28340, t689, t780, t103182, t93281, t103421, t7058, t25317, t25383, t26475, t28385, t28405, t28417, t28436, t7070, t7415, t7766, t8012, t886, t92917, t93126, t95930, t95937, t95945, t95948, t99303);
        let t103553 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2028::<F>(t102954, t102977, t103008, t103033, t103065, t103100, t103137, t103166, t103210, t103242, t103380, t103412, t103451, t103488, t103519, t103549);
        let (t103554, t103561, t103570, t103574) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2029::<F>(t103553, t892, t26425, t98648, t1940, t2255, t7428, t102917, t2071, t2403, t25215, t26585, t27173, t27387, t28291, t28472, t30, t4541, t7432, t8020, t98652, t98675, t98705, t98709, t98736, t98780, t98793, t99543);
        let (t103586, t103612) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2030::<F>(t11064, t8019, t1940, t2071, t2403, t25446, t26425, t26581, t26585, t26590, t27376, t27391, t28456, t28472, t51780, t7010, t7432, t7749, t7991, t95511, t98627, t98659, t98662, t98740, t98743, t98751, t98755, t98768, t99550);
        let t103658 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2031::<F>(t100858, t103553, t14749, t14767, t15071, t1544, t1583, t1940, t198, t207, t2071, t2394, t2403, t26425, t26581, t26590, t28291, t2832, t28460, t4343, t4433, t4541, t61155, t61182, t63186, t7428, t7432, t8020, t892, t95527, t95964, t98759, t98786);
        let t103706 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2032::<F>(t102854, t103586, t14365, t14468, t18875, t1940, t2071, t2403, t2408, t2430, t26585, t26590, t27375, t27384, t28456, t28460, t4537, t51780, t61102, t61203, t63164, t7432, t775, t8020, t8031, t890, t95976, t98651, t98779);
        let (t103707, t103720) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2033::<F>(t30, t265, t393, t103658, t103706, t102867, t102905, t103574, t103612, t13312, t1469, t2078, t2258, t26626, t28523, t4186, t45, t606, t7449, t8040, dens_threshold, rho0, zeta_threshold);
        let t103750 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2034::<F>(t100944, t100947, t100953, t100958, t100969, t100978, t101029, t101032, t101086, t102851, t102858, t1711, t1940, t26425, t26581, t27793, t27800, t28291, t3351, t7432, t8020, t95511);
        let t103778 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2035::<F>(t100964, t100975, t100982, t101016, t101065, t101093, t102854, t102864, t102877, t102888, t102917, t103586, t1940, t2403, t25760, t25763, t25778, t26425, t26585, t27764, t27806, t27817, t28472, t7207, t7432, t8020);
        let t103817 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2036::<F>(t100993, t100997, t101035, t101040, t101051, t101061, t101070, t101074, t103554, t103561, t1113, t1940, t2071, t2403, t25752, t25767, t26425, t26590, t27773, t28291, t28456, t33, t4541, t51780, t7428, t8020, t8046);
        let t103853 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2037::<F>(t100988, t101012, t101055, t101083, t101099, t103570, t1940, t2403, t25781, t25784, t26425, t26581, t26585, t27770, t27777, t27802, t27810, t28456, t28460, t7200, t7428, t7432, t7862, t7869, t95511, t95527);
        let t103868 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2038::<F>(t33, t265, t502, t103707, t103750, t103778, t103817, t103853, t13312, t1469, t2085, t2258, t26666, t28578, t4186, t57, t606, t7468, t8059, dens_threshold, rho1, zeta_threshold);
    (t103720, t103868)
}
