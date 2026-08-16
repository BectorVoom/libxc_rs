//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2141;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2142;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2143;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta660(t18657: f64, t1955: f64, t18797: f64, t25399: f64, t1579: f64, t231: f64, t4423: f64, t1580: f64, t27194: f64, t689: f64, t29690: f64, t25411: f64, t25431: f64, t18313: f64, t18663: f64, t1959: f64, t25317: f64, t25391: f64, t25392: f64, t27265: f64, t29636: f64, t6048: f64, t7048: f64, t7053: f64, t7070: f64, t7071: f64, t886: f64, t93334: f64, t93335: f64, t93339: f64, t99456: f64, t27341: f64, t99463: f64, t99466: f64, t14495: f64, t27199: f64, t27287: f64, t29659: f64, t4533: f64, t7067: f64, t7759: f64, t93372: f64, t99414: f64, t99460: f64, t99465: f64, t99468: f64, t99472: f64, t99475: f64, t99480: f64, t99481: f64, t25383: f64, t29644: f64, t7769: f64, t7770: f64, t93378: f64, t93382: f64, t93384: f64, t93391: f64, t99303: f64, t99485: f64, t99487: f64, t99491: f64, t99493: f64, t99496: f64, t99502: f64, t99520: f64, t99522: f64, t105958: f64, t105969: f64, t106116: f64, t106134: f64, t106164: f64, t106190: f64, t106215: f64, t106245: f64, t106284: f64, t106313: f64, t106342: f64, t106382: f64, t106403: f64, t892: f64, t198: f64, t205: f64, t7782: f64, t25207: f64, t77441: f64, t1544: f64, t580: f64, t98646: f64, t25206: f64, t105898: f64, t105902: f64, t105906: f64, t105909: f64, t105919: f64, t105924: f64, t105930: f64, t1940: f64, t1963: f64, t2403: f64, t25440: f64, t27160: f64, t29591: f64, t29602: f64, t29716: f64, t30: f64, t4541: f64, t7087: f64, t18435: f64, t27159: f64, t29598: f64, t890: f64, t27383: f64, t18838: f64, t2411: f64, t29704: f64, t18875: f64, t98658: f64, t92790: f64, t775: f64, t2255: f64, t7783: f64, t77425: f64, t1468: f64, t27158: f64, t27166: f64, t27173: f64, t27364: f64, t27368: f64, t27391: f64, t29705: f64, t605: f64, t7091: f64, t7092: f64, t7787: f64, t98637: f64, t99555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106404, t106407, t106410, t106423, t106430, t106431) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137(t18657, t1955, t18797, t25399, t1579, t231, t4423, t1580, t27194, t689, t29690, t25411);
        let t106441 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2138(t106430, t25431, t106404, t106407, t106410, t106423, t106431, t1579, t18313, t18663, t1959, t25317, t25391, t25392, t27265, t29636, t6048, t7048, t7053, t7070, t7071, t886, t93334, t93335, t93339, t99456);
        let t106461 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2139(t27341, t99463, t99466, t14495, t25391, t27199, t27287, t29659, t4533, t7067, t7070, t7071, t7759, t93372, t99414, t99460, t99465, t99468, t99472, t99475, t99480, t99481);
        let t106477 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140(t25317, t25383, t29644, t4533, t7070, t7769, t7770, t93378, t93382, t93384, t93391, t99303, t99485, t99487, t99491, t99493, t99496, t99502, t99520, t99522);
        let t106481 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2141(t105958, t105969, t106116, t106134, t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403, t106441, t106461, t106477);
        let (t106482, t106487, t106496, t106497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2142(t106481, t892, t198, t205, t7782, t25207, t77441, t1544, t580, t98646, t25206, t105898, t105902, t105906, t105909, t105919, t105924, t105930, t1940, t1963, t2403, t25440, t27160, t29591, t29602, t29716, t30, t4541, t7087);
        let (t106498, t106501, t106502, t106510, t106516, t106520, t106528) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2143(t18435, t27159, t29598, t890, t27383, t18838, t30, t2411, t29704, t18875, t98658, t92790);
        let (t106533, t106539, t106543) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2144(t29598, t775, t25207, t1940, t2255, t7783, t77425, t106498, t106502, t106510, t106516, t106520, t106528, t1468, t2403, t25206, t27158, t27166, t27173, t27364, t27368, t27391, t29705, t605, t7091, t7092, t7787, t98637, t99555);
    (t106481, t106482, t106487, t106496, t106497, t106501, t106516, t106533, t106539, t106543)
}
