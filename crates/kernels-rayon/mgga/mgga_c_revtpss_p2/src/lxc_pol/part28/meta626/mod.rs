//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta626 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2235;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2236;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2237;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2238;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2239;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2240;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2241;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2242;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2243;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2244;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2245;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta626(t100690: f64, t7150: f64, t7810: f64, t989: f64, t25698: f64, t27418: f64, t4746: f64, t7135: f64, t1982: f64, t99708: f64, t3047: f64, t8521: f64, t1096: f64, t1646: f64, t1000: f64, t100490: f64, t1097: f64, t16275: f64, t1985: f64, t25473: f64, t25591: f64, t25701: f64, t27412: f64, t27433: f64, t27653: f64, t27668: f64, t27669: f64, t27670: f64, t27679: f64, t27702: f64, t3042: f64, t3318: f64, t4983: f64, t64841: f64, t7144: f64, t7145: f64, t7156: f64, t7160: f64, t7162: f64, t7828: f64, t93498: f64, t93502: f64, t93921: f64, t94026: f64, t94042: f64, t94063: f64, t99730: f64, t999: f64, t15654: f64, t1976: f64, t100403: f64, t15648: f64, t225: f64, t25464: f64, t25476: f64, t25597: f64, t25629: f64, t25658: f64, t25695: f64, t27419: f64, t27426: f64, t27441: f64, t27550: f64, t27595: f64, t27651: f64, t27652: f64, t27695: f64, t27699: f64, t3067: f64, t3271: f64, t342: f64, t385: f64, t4772: f64, t4773: f64, t4947: f64, t4975: f64, t7151: f64, t7159: f64, t7822: f64, t93429: f64, t94016: f64, t99762: f64, t27708: f64, t3336: f64, t11108: f64, t7840: f64, t100425: f64, t100471: f64, t100513: f64, t100560: f64, t100606: f64, t100650: f64, t100696: f64, t1100: f64, t1102: f64, t16612: f64, t1699: f64, t198: f64, t25709: f64, t25713: f64, t27712: f64, t27717: f64, t3329: f64, t3333: f64, t336: f64, t5019: f64, t5023: f64, t63827: f64, t7181: f64, t94138: f64, t94142: f64, t94149: f64, t99618: f64, t99673: f64, t99728: f64, t99790: f64, t99847: f64, t99901: f64, t99950: f64, t14365: f64, t14436: f64, t14468: f64, t14749: f64, t14767: f64, t1940: f64, t1963: f64, t207: f64, t2394: f64, t2403: f64, t2408: f64, t25206: f64, t25445: f64, t27368: f64, t27384: f64, t4433: f64, t4541: f64, t61155: f64, t61182: f64, t63164: f64, t7087: f64, t7091: f64, t7783: f64, t892: f64, t92742: f64, t93404: f64, t98722: f64, t98759: f64, t98779: f64, t98786: f64, t99536: f64, t15071: f64, t1544: f64, t1583: f64, t18875: f64, t2430: f64, t25436: f64, t25440: f64, t27158: f64, t27364: f64, t27375: f64, t2832: f64, t4343: f64, t4537: f64, t51780: f64, t61102: f64, t61203: f64, t63186: f64, t775: f64, t7847: f64, t890: f64, t92775: f64, t98651: f64, t99555: f64, t30: f64, t265: f64, t393: f64, t13312: f64, t1469: f64, t1996: f64, t2258: f64, t25744: f64, t27755: f64, t4186: f64, t45: f64, t606: f64, t7194: f64, t7856: f64, t99565: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t94245: f64, t25759: f64, t98674: f64, t33: f64, t25781: f64, t27764: f64, t3351: f64, t7200: f64, t98635: f64, t98650: f64, t98669: f64, t98684: f64, t99537: f64, t11064: f64, t1113: f64, t27799: f64, t98767: f64, t41154: f64, t1711: f64, t2411: f64, t25752: f64, t25760: f64, t25784: f64, t27382: f64, t27770: f64, t27793: f64, t27806: f64, t7869: f64, t92819: f64, t98637: f64, t27763: f64, t25763: f64, t25778: f64, t27773: f64, t27800: f64, t7207: f64, t7862: f64, t98719: f64, t98784: f64) -> (f64, f64, f64, f64, f64) {
        let (t100698, t100702, t100705, t100708, t100723, t100737) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2235(t100690, t7150, t7810, t989, t25698, t27418, t4746, t7135, t1982, t99708, t3047, t8521);
        let t100748 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2236(t1096, t1646, t1000, t100490, t100698, t100702, t100705, t100708, t100723, t100737, t1097, t16275, t1985, t25473, t25591, t25701, t27412, t27433, t27653, t27668, t27669, t27670, t27679, t27702, t3042, t3318, t4983, t64841, t7144, t7145, t7156, t7160, t7162, t7828, t93498, t93502, t93921, t94026, t94042, t94063, t99730, t999);
        let t100794 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2237(t15654, t1976, t1000, t100403, t1096, t15648, t225, t25464, t25473, t25476, t25597, t25629, t25658, t25695, t27419, t27426, t27441, t27550, t27595, t27651, t27652, t27695, t27699, t3042, t3067, t3271, t342, t385, t4772, t4773, t4947, t4975, t7135, t7145, t7151, t7159, t7822, t93429, t93498, t94016, t99762);
        let t100833 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2238(t27708, t3336, t11108, t7840, t100425, t100471, t100513, t100560, t100606, t100650, t100696, t100748, t100794, t1100, t1102, t16612, t1699, t198, t25709, t25713, t27712, t27717, t3329, t3333, t336, t5019, t5023, t63827, t7181, t94138, t94142, t94149, t99618, t99673, t99728, t99790, t99847, t99901, t99950);
        let t100882 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2239(t14365, t14436, t14468, t14749, t14767, t1940, t1963, t198, t207, t2394, t2403, t2408, t25206, t25445, t27368, t27384, t4433, t4541, t61155, t61182, t63164, t7087, t7091, t7783, t892, t92742, t93404, t98722, t98759, t98779, t98786, t99536);
        let t100926 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2240(t15071, t1544, t1583, t18875, t1940, t2403, t2430, t25436, t25440, t27158, t27364, t27368, t27375, t2832, t4343, t4537, t51780, t61102, t61203, t63186, t7087, t7091, t775, t7783, t7847, t890, t92775, t98651, t99555);
        let (t100927, t100940) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2241(t30, t265, t393, t100882, t100926, t100833, t13312, t1469, t1996, t2258, t25744, t27755, t4186, t45, t606, t7194, t7856, t99565, dens_threshold, rho0, zeta_threshold);
        let t100973 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2242(t18875, t94245, t25759, t61203, t98674, t98759, t98651, t15071, t33, t1940, t2403, t25206, t25781, t27158, t27364, t27368, t27764, t3351, t7091, t7200, t7783, t98635, t98650, t98669, t98684, t99537);
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2243(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let t101021 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2244(t1113, t4343, t1583, t3351, t27799, t63164, t100975, t100978, t100982, t100988, t100993, t1940, t1963, t2403, t25206, t25440, t25752, t25760, t25784, t27368, t27382, t27770, t27793, t27806, t4541, t7091, t7783, t7869, t92775, t92819, t98637);
        let (t101029, t101032, t101035, t101040, t101051, t101055) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2245(t1113, t4433, t892, t14749, t27763, t14767, t1711, t2408, t14468, t33, t25759, t61102);
        let t101064 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2246(t25759, t61182, t101029, t101032, t101035, t101040, t101051, t101055, t1711, t1940, t1963, t2403, t25206, t25436, t25445, t25763, t25778, t27158, t27773, t27800, t7087, t7207, t7783, t7862, t98719, t98722, t98784, t99555);
    (t100927, t100940, t100973, t101021, t101064)
}
