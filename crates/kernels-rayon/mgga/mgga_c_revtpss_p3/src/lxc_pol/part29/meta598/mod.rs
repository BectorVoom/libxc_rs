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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta598(t103431: f64, t25375: f64, t212: f64, t28340: f64, t689: f64, t780: f64, t103182: f64, t93281: f64, t103421: f64, t7058: f64, t25317: f64, t25383: f64, t26475: f64, t28385: f64, t28405: f64, t28417: f64, t28436: f64, t7070: f64, t7415: f64, t7766: f64, t8012: f64, t886: f64, t92917: f64, t93126: f64, t95930: f64, t95937: f64, t95945: f64, t95948: f64, t99303: f64, t102954: f64, t102977: f64, t103008: f64, t103033: f64, t103065: f64, t103100: f64, t103137: f64, t103166: f64, t103210: f64, t103242: f64, t103380: f64, t103412: f64, t103451: f64, t103488: f64, t103519: f64, t892: f64, t26425: f64, t98648: f64, t1940: f64, t2255: f64, t7428: f64, t102917: f64, t2071: f64, t2403: f64, t25215: f64, t26585: f64, t27173: f64, t27387: f64, t28291: f64, t28472: f64, t30: f64, t4541: f64, t7432: f64, t8020: f64, t98652: f64, t98675: f64, t98705: f64, t98709: f64, t98736: f64, t98780: f64, t98793: f64, t99543: f64, t11064: f64, t8019: f64, t25446: f64, t26581: f64, t26590: f64, t27376: f64, t27391: f64, t28456: f64, t51780: f64, t7010: f64, t7749: f64, t7991: f64, t95511: f64, t98627: f64, t98659: f64, t98662: f64, t98740: f64, t98743: f64, t98751: f64, t98755: f64, t98768: f64, t99550: f64, t100858: f64, t14749: f64, t14767: f64, t15071: f64, t1544: f64, t1583: f64, t198: f64, t207: f64, t2394: f64, t2832: f64, t28460: f64, t4343: f64, t4433: f64, t61155: f64, t61182: f64, t63186: f64, t95527: f64, t95964: f64, t98759: f64, t98786: f64, t102854: f64, t14365: f64, t14468: f64, t18875: f64, t2408: f64, t2430: f64, t27375: f64, t27384: f64, t4537: f64, t61102: f64, t61203: f64, t63164: f64, t775: f64, t8031: f64, t890: f64, t95976: f64, t98651: f64, t98779: f64, t265: f64, t393: f64, t102867: f64, t102905: f64, t13312: f64, t1469: f64, t2078: f64, t2258: f64, t26626: f64, t28523: f64, t4186: f64, t45: f64, t606: f64, t7449: f64, t8040: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t100944: f64, t100947: f64, t100953: f64, t100958: f64, t100969: f64, t100978: f64, t101029: f64, t101032: f64, t101086: f64, t102851: f64, t102858: f64, t1711: f64, t27793: f64, t27800: f64, t3351: f64, t100964: f64, t100975: f64, t100982: f64, t101016: f64, t101065: f64, t101093: f64, t102864: f64, t102877: f64, t102888: f64, t25760: f64, t25763: f64, t25778: f64, t27764: f64, t27806: f64, t27817: f64, t7207: f64, t100993: f64, t100997: f64, t101035: f64, t101040: f64, t101051: f64, t101061: f64, t101070: f64, t101074: f64, t1113: f64, t25752: f64, t25767: f64, t27773: f64, t33: f64, t8046: f64, t100988: f64, t101012: f64, t101055: f64, t101083: f64, t101099: f64, t25781: f64, t25784: f64, t27770: f64, t27777: f64, t27802: f64, t27810: f64, t7200: f64, t7862: f64, t7869: f64, t502: f64, t2085: f64, t26666: f64, t28578: f64, t57: f64, t7468: f64, t8059: f64, rho1: f64) -> (f64, f64) {
        let t103549 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2027(t103431, t25375, t212, t28340, t689, t780, t103182, t93281, t103421, t7058, t25317, t25383, t26475, t28385, t28405, t28417, t28436, t7070, t7415, t7766, t8012, t886, t92917, t93126, t95930, t95937, t95945, t95948, t99303);
        let t103553 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2028(t102954, t102977, t103008, t103033, t103065, t103100, t103137, t103166, t103210, t103242, t103380, t103412, t103451, t103488, t103519, t103549);
        let (t103554, t103561, t103570, t103574) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2029(t103553, t892, t26425, t98648, t1940, t2255, t7428, t102917, t2071, t2403, t25215, t26585, t27173, t27387, t28291, t28472, t30, t4541, t7432, t8020, t98652, t98675, t98705, t98709, t98736, t98780, t98793, t99543);
        let (t103586, t103612) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2030(t11064, t8019, t1940, t2071, t2403, t25446, t26425, t26581, t26585, t26590, t27376, t27391, t28456, t28472, t51780, t7010, t7432, t7749, t7991, t95511, t98627, t98659, t98662, t98740, t98743, t98751, t98755, t98768, t99550);
        let t103658 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2031(t100858, t103553, t14749, t14767, t15071, t1544, t1583, t1940, t198, t207, t2071, t2394, t2403, t26425, t26581, t26590, t28291, t2832, t28460, t4343, t4433, t4541, t61155, t61182, t63186, t7428, t7432, t8020, t892, t95527, t95964, t98759, t98786);
        let t103706 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2032(t102854, t103586, t14365, t14468, t18875, t1940, t2071, t2403, t2408, t2430, t26585, t26590, t27375, t27384, t28456, t28460, t4537, t51780, t61102, t61203, t63164, t7432, t775, t8020, t8031, t890, t95976, t98651, t98779);
        let (t103707, t103720) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2033(t30, t265, t393, t103658, t103706, t102867, t102905, t103574, t103612, t13312, t1469, t2078, t2258, t26626, t28523, t4186, t45, t606, t7449, t8040, dens_threshold, rho0, zeta_threshold);
        let t103750 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2034(t100944, t100947, t100953, t100958, t100969, t100978, t101029, t101032, t101086, t102851, t102858, t1711, t1940, t26425, t26581, t27793, t27800, t28291, t3351, t7432, t8020, t95511);
        let t103778 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2035(t100964, t100975, t100982, t101016, t101065, t101093, t102854, t102864, t102877, t102888, t102917, t103586, t1940, t2403, t25760, t25763, t25778, t26425, t26585, t27764, t27806, t27817, t28472, t7207, t7432, t8020);
        let t103817 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2036(t100993, t100997, t101035, t101040, t101051, t101061, t101070, t101074, t103554, t103561, t1113, t1940, t2071, t2403, t25752, t25767, t26425, t26590, t27773, t28291, t28456, t33, t4541, t51780, t7428, t8020, t8046);
        let t103853 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2037(t100988, t101012, t101055, t101083, t101099, t103570, t1940, t2403, t25781, t25784, t26425, t26581, t26585, t27770, t27777, t27802, t27810, t28456, t28460, t7200, t7428, t7432, t7862, t7869, t95511, t95527);
        let t103868 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2038(t33, t265, t502, t103707, t103750, t103778, t103817, t103853, t13312, t1469, t2085, t2258, t26666, t28578, t4186, t57, t606, t7468, t8059, dens_threshold, rho1, zeta_threshold);
    (t103720, t103868)
}
