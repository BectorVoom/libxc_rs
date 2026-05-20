//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta661 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2145;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2146;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2147;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2148;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2149;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2150;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2151;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2152;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2153;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2154;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta661<F: Float>(t1468: F, t4433: F, t892: F, t1583: F, t4537: F, t27383: F, t6079: F, t775: F, t890: F, t98785: F, t25207: F, t77408: F, t18498: F, t27159: F, t1940: F, t2403: F, t25206: F, t25440: F, t27158: F, t27364: F, t27368: F, t27382: F, t27395: F, t27402: F, t29592: F, t29606: F, t29713: F, t29719: F, t50080: F, t7087: F, t7091: F, t7749: F, t7783: F, t93404: F, t11064: F, t27384: F, t605: F, t198: F, t7850: F, t5824: F, t6075: F, t18392: F, t30: F, t4343: F, t18280: F, t1963: F, t25445: F, t27169: F, t27376: F, t27385: F, t27387: F, t29599: F, t29705: F, t7010: F, t92819: F, t98637: F, t106497: F, t106543: F, t27375: F, t63185: F, t1544: F, t105923: F, t106481: F, t106516: F, t207: F, t29598: F, t4541: F, t5962: F, t98722: F, t99555: F, t18435: F, t18838: F, t18875: F, t29907: F, t5966: F, t77425: F, t77441: F, t92742: F, t25759: F, t100987: F, t94245: F, t1711: F, t27799: F, t20256: F, t27770: F, t27793: F, t27800: F, t29939: F, t29949: F, t29953: F, t106533: F, t1113: F, t33: F, t100981: F, t105930: F, t106487: F, t106496: F, t27764: F, t27802: F, t27806: F, t29970: F, t6416: F, t106501: F, t106539: F, t27773: F, t27777: F, t29940: F, t29946: F, t29967: F, t7200: F, t7862: F, t7869: F) -> (F, F, F, F, F, F, F) {
        let (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2145::<F>(t1468, t4433, t892, t1583, t4537, t27383, t6079, t775, t890, t98785, t25207, t77408);
        let t106588 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2146::<F>(t18498, t27159, t1468, t4537, t106546, t106555, t106562, t106566, t106569, t1940, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27395, t27402, t29592, t29606, t29713, t29719, t50080, t7087, t7091, t7749, t7783, t93404);
        let (t106590, t106593, t106596, t106602, t106606, t106610, t106611, t106618) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2147::<F>(t11064, t1468, t27384, t605, t6079, t198, t7850, t5824, t890, t6075, t27383, t18392, t30);
        let (t106625, t106636) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2148::<F>(t1583, t4343, t25207, t106590, t106593, t106596, t106602, t106606, t106611, t106618, t18280, t1940, t1963, t2403, t25206, t25445, t27169, t27368, t27376, t27382, t27385, t27387, t29599, t29705, t5824, t7010, t7087, t7091, t7783, t92819, t98637);
        let (t106638, t107820) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2149::<F>(t106497, t106543, t106588, t106636, t27375, t63185, t11064, t1544, t27384, t105923, t106481, t106516, t106610, t1583, t18392, t18498, t1940, t1963, t198, t207, t2403, t25206, t25440, t25445, t27158, t29598, t4343, t4433, t4541, t5962, t6075, t7087, t7091, t77408, t7783, t890, t892, t98722, t99555);
        let t107867 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2150::<F>(t106554, t106561, t106565, t106625, t1544, t18435, t18838, t18875, t1940, t1963, t2403, t25445, t27364, t27368, t27375, t29705, t29907, t4537, t4541, t50080, t5966, t6079, t7087, t7091, t77425, t77441, t775, t92742, t93404);
        let (t107868, t107882, t107885, t107892, t107895, t107901, t107908) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2151::<F>(t107820, t107867, t25759, t77425, t100987, t27375, t106625, t29598, t94245, t1711, t4343, t106561, t27799);
        let t107922 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2152::<F>(t105923, t25759, t106596, t107882, t107885, t107892, t107895, t107901, t107908, t1940, t1963, t20256, t2403, t25206, t27770, t27793, t27800, t29939, t29949, t29953, t4541, t7087, t98637);
        let (t107924, t107927, t107930, t107934, t107939, t107943) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2153::<F>(t11064, t1711, t27384, t106533, t25759, t100987, t18875, t4433, t892, t1113, t5962, t18392, t33);
        let t107963 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2154::<F>(t100981, t106565, t1113, t6079, t105930, t106487, t106496, t107924, t107927, t107930, t107934, t107939, t107943, t1940, t1963, t2403, t25206, t25440, t25445, t27158, t27368, t27382, t27764, t27802, t27806, t29970, t6416, t7087);
        let t108001 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2155::<F>(t6416, t775, t106501, t27799, t25759, t77441, t1711, t4537, t106539, t1113, t1940, t1963, t2403, t25206, t25440, t27364, t27773, t27777, t29705, t29940, t29946, t29967, t50080, t7091, t7200, t7783, t7862, t7869, t92819, t99555);
    (t106554, t106610, t106638, t107868, t107922, t107963, t108001)
}
