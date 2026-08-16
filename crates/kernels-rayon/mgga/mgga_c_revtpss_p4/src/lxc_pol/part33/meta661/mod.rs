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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta661(t1468: f64, t4433: f64, t892: f64, t1583: f64, t4537: f64, t27383: f64, t6079: f64, t775: f64, t890: f64, t98785: f64, t25207: f64, t77408: f64, t18498: f64, t27159: f64, t1940: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27395: f64, t27402: f64, t29592: f64, t29606: f64, t29713: f64, t29719: f64, t50080: f64, t7087: f64, t7091: f64, t7749: f64, t7783: f64, t93404: f64, t11064: f64, t27384: f64, t605: f64, t198: f64, t7850: f64, t5824: f64, t6075: f64, t18392: f64, t30: f64, t4343: f64, t18280: f64, t1963: f64, t25445: f64, t27169: f64, t27376: f64, t27385: f64, t27387: f64, t29599: f64, t29705: f64, t7010: f64, t92819: f64, t98637: f64, t106497: f64, t106543: f64, t27375: f64, t63185: f64, t1544: f64, t105923: f64, t106481: f64, t106516: f64, t207: f64, t29598: f64, t4541: f64, t5962: f64, t98722: f64, t99555: f64, t18435: f64, t18838: f64, t18875: f64, t29907: f64, t5966: f64, t77425: f64, t77441: f64, t92742: f64, t25759: f64, t100987: f64, t94245: f64, t1711: f64, t27799: f64, t20256: f64, t27770: f64, t27793: f64, t27800: f64, t29939: f64, t29949: f64, t29953: f64, t106533: f64, t1113: f64, t33: f64, t100981: f64, t105930: f64, t106487: f64, t106496: f64, t27764: f64, t27802: f64, t27806: f64, t29970: f64, t6416: f64, t106501: f64, t106539: f64, t27773: f64, t27777: f64, t29940: f64, t29946: f64, t29967: f64, t7200: f64, t7862: f64, t7869: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t106546, t106554, t106555, t106561, t106562, t106565, t106566, t106569) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2145(t1468, t4433, t892, t1583, t4537, t27383, t6079, t775, t890, t98785, t25207, t77408);
        let t106588 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2146(t18498, t27159, t1468, t4537, t106546, t106555, t106562, t106566, t106569, t1940, t2403, t25206, t25440, t27158, t27364, t27368, t27382, t27395, t27402, t29592, t29606, t29713, t29719, t50080, t7087, t7091, t7749, t7783, t93404);
        let (t106590, t106593, t106596, t106602, t106606, t106610, t106611, t106618) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2147(t11064, t1468, t27384, t605, t6079, t198, t7850, t5824, t890, t6075, t27383, t18392, t30);
        let (t106625, t106636) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2148(t1583, t4343, t25207, t106590, t106593, t106596, t106602, t106606, t106611, t106618, t18280, t1940, t1963, t2403, t25206, t25445, t27169, t27368, t27376, t27382, t27385, t27387, t29599, t29705, t5824, t7010, t7087, t7091, t7783, t92819, t98637);
        let (t106638, t107820) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2149(t106497, t106543, t106588, t106636, t27375, t63185, t11064, t1544, t27384, t105923, t106481, t106516, t106610, t1583, t18392, t18498, t1940, t1963, t198, t207, t2403, t25206, t25440, t25445, t27158, t29598, t4343, t4433, t4541, t5962, t6075, t7087, t7091, t77408, t7783, t890, t892, t98722, t99555);
        let t107867 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2150(t106554, t106561, t106565, t106625, t1544, t18435, t18838, t18875, t1940, t1963, t2403, t25445, t27364, t27368, t27375, t29705, t29907, t4537, t4541, t50080, t5966, t6079, t7087, t7091, t77425, t77441, t775, t92742, t93404);
        let (t107868, t107882, t107885, t107892, t107895, t107901, t107908) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2151(t107820, t107867, t25759, t77425, t100987, t27375, t106625, t29598, t94245, t1711, t4343, t106561, t27799);
        let t107922 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2152(t105923, t25759, t106596, t107882, t107885, t107892, t107895, t107901, t107908, t1940, t1963, t20256, t2403, t25206, t27770, t27793, t27800, t29939, t29949, t29953, t4541, t7087, t98637);
        let (t107924, t107927, t107930, t107934, t107939, t107943) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2153(t11064, t1711, t27384, t106533, t25759, t100987, t18875, t4433, t892, t1113, t5962, t18392, t33);
        let t107963 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2154(t100981, t106565, t1113, t6079, t105930, t106487, t106496, t107924, t107927, t107930, t107934, t107939, t107943, t1940, t1963, t2403, t25206, t25440, t25445, t27158, t27368, t27382, t27764, t27802, t27806, t29970, t6416, t7087);
        let t108001 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2155(t6416, t775, t106501, t27799, t25759, t77441, t1711, t4537, t106539, t1113, t1940, t1963, t2403, t25206, t25440, t27364, t27773, t27777, t29705, t29940, t29946, t29967, t50080, t7091, t7200, t7783, t7862, t7869, t92819, t99555);
    (t106554, t106610, t106638, t107868, t107922, t107963, t108001)
}
