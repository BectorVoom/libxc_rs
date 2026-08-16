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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta626<F: Float>(t100690: F, t7150: F, t7810: F, t989: F, t25698: F, t27418: F, t4746: F, t7135: F, t1982: F, t99708: F, t3047: F, t8521: F, t1096: F, t1646: F, t1000: F, t100490: F, t1097: F, t16275: F, t1985: F, t25473: F, t25591: F, t25701: F, t27412: F, t27433: F, t27653: F, t27668: F, t27669: F, t27670: F, t27679: F, t27702: F, t3042: F, t3318: F, t4983: F, t64841: F, t7144: F, t7145: F, t7156: F, t7160: F, t7162: F, t7828: F, t93498: F, t93502: F, t93921: F, t94026: F, t94042: F, t94063: F, t99730: F, t999: F, t15654: F, t1976: F, t100403: F, t15648: F, t225: F, t25464: F, t25476: F, t25597: F, t25629: F, t25658: F, t25695: F, t27419: F, t27426: F, t27441: F, t27550: F, t27595: F, t27651: F, t27652: F, t27695: F, t27699: F, t3067: F, t3271: F, t342: F, t385: F, t4772: F, t4773: F, t4947: F, t4975: F, t7151: F, t7159: F, t7822: F, t93429: F, t94016: F, t99762: F, t27708: F, t3336: F, t11108: F, t7840: F, t100425: F, t100471: F, t100513: F, t100560: F, t100606: F, t100650: F, t100696: F, t1100: F, t1102: F, t16612: F, t1699: F, t198: F, t25709: F, t25713: F, t27712: F, t27717: F, t3329: F, t3333: F, t336: F, t5019: F, t5023: F, t63827: F, t7181: F, t94138: F, t94142: F, t94149: F, t99618: F, t99673: F, t99728: F, t99790: F, t99847: F, t99901: F, t99950: F, t14365: F, t14436: F, t14468: F, t14749: F, t14767: F, t1940: F, t1963: F, t207: F, t2394: F, t2403: F, t2408: F, t25206: F, t25445: F, t27368: F, t27384: F, t4433: F, t4541: F, t61155: F, t61182: F, t63164: F, t7087: F, t7091: F, t7783: F, t892: F, t92742: F, t93404: F, t98722: F, t98759: F, t98779: F, t98786: F, t99536: F, t15071: F, t1544: F, t1583: F, t18875: F, t2430: F, t25436: F, t25440: F, t27158: F, t27364: F, t27375: F, t2832: F, t4343: F, t4537: F, t51780: F, t61102: F, t61203: F, t63186: F, t775: F, t7847: F, t890: F, t92775: F, t98651: F, t99555: F, t30: F, t265: F, t393: F, t13312: F, t1469: F, t1996: F, t2258: F, t25744: F, t27755: F, t4186: F, t45: F, t606: F, t7194: F, t7856: F, t99565: F, dens_threshold: F, rho0: F, zeta_threshold: F, t94245: F, t25759: F, t98674: F, t33: F, t25781: F, t27764: F, t3351: F, t7200: F, t98635: F, t98650: F, t98669: F, t98684: F, t99537: F, t11064: F, t1113: F, t27799: F, t98767: F, t41154: F, t1711: F, t2411: F, t25752: F, t25760: F, t25784: F, t27382: F, t27770: F, t27793: F, t27806: F, t7869: F, t92819: F, t98637: F, t27763: F, t25763: F, t25778: F, t27773: F, t27800: F, t7207: F, t7862: F, t98719: F, t98784: F) -> (F, F, F, F, F) {
        let (t100698, t100702, t100705, t100708, t100723, t100737) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2235::<F>(t100690, t7150, t7810, t989, t25698, t27418, t4746, t7135, t1982, t99708, t3047, t8521);
        let t100748 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2236::<F>(t1096, t1646, t1000, t100490, t100698, t100702, t100705, t100708, t100723, t100737, t1097, t16275, t1985, t25473, t25591, t25701, t27412, t27433, t27653, t27668, t27669, t27670, t27679, t27702, t3042, t3318, t4983, t64841, t7144, t7145, t7156, t7160, t7162, t7828, t93498, t93502, t93921, t94026, t94042, t94063, t99730, t999);
        let t100794 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2237::<F>(t15654, t1976, t1000, t100403, t1096, t15648, t225, t25464, t25473, t25476, t25597, t25629, t25658, t25695, t27419, t27426, t27441, t27550, t27595, t27651, t27652, t27695, t27699, t3042, t3067, t3271, t342, t385, t4772, t4773, t4947, t4975, t7135, t7145, t7151, t7159, t7822, t93429, t93498, t94016, t99762);
        let t100833 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2238::<F>(t27708, t3336, t11108, t7840, t100425, t100471, t100513, t100560, t100606, t100650, t100696, t100748, t100794, t1100, t1102, t16612, t1699, t198, t25709, t25713, t27712, t27717, t3329, t3333, t336, t5019, t5023, t63827, t7181, t94138, t94142, t94149, t99618, t99673, t99728, t99790, t99847, t99901, t99950);
        let t100882 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2239::<F>(t14365, t14436, t14468, t14749, t14767, t1940, t1963, t198, t207, t2394, t2403, t2408, t25206, t25445, t27368, t27384, t4433, t4541, t61155, t61182, t63164, t7087, t7091, t7783, t892, t92742, t93404, t98722, t98759, t98779, t98786, t99536);
        let t100926 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2240::<F>(t15071, t1544, t1583, t18875, t1940, t2403, t2430, t25436, t25440, t27158, t27364, t27368, t27375, t2832, t4343, t4537, t51780, t61102, t61203, t63186, t7087, t7091, t775, t7783, t7847, t890, t92775, t98651, t99555);
        let (t100927, t100940) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2241::<F>(t30, t265, t393, t100882, t100926, t100833, t13312, t1469, t1996, t2258, t25744, t27755, t4186, t45, t606, t7194, t7856, t99565, dens_threshold, rho0, zeta_threshold);
        let t100973 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2242::<F>(t18875, t94245, t25759, t61203, t98674, t98759, t98651, t15071, t33, t1940, t2403, t25206, t25781, t27158, t27364, t27368, t27764, t3351, t7091, t7200, t7783, t98635, t98650, t98669, t98684, t99537);
        let (t100975, t100978, t100982, t100988, t100993) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2243::<F>(t11064, t1113, t27384, t27799, t98767, t33, t41154, t98786, t1711, t2411, t14365, t1544, t3351);
        let t101021 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2244::<F>(t1113, t4343, t1583, t3351, t27799, t63164, t100975, t100978, t100982, t100988, t100993, t1940, t1963, t2403, t25206, t25440, t25752, t25760, t25784, t27368, t27382, t27770, t27793, t27806, t4541, t7091, t7783, t7869, t92775, t92819, t98637);
        let (t101029, t101032, t101035, t101040, t101051, t101055) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2245::<F>(t1113, t4433, t892, t14749, t27763, t14767, t1711, t2408, t14468, t33, t25759, t61102);
        let t101064 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2246::<F>(t25759, t61182, t101029, t101032, t101035, t101040, t101051, t101055, t1711, t1940, t1963, t2403, t25206, t25436, t25445, t25763, t25778, t27158, t27773, t27800, t7087, t7207, t7783, t7862, t98719, t98722, t98784, t99555);
    (t100927, t100940, t100973, t101021, t101064)
}
