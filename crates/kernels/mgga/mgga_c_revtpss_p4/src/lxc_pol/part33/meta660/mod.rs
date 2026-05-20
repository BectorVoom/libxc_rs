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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2141;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2142;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2143;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta660<F: Float>(t18657: F, t1955: F, t18797: F, t25399: F, t1579: F, t231: F, t4423: F, t1580: F, t27194: F, t689: F, t29690: F, t25411: F, t25431: F, t18313: F, t18663: F, t1959: F, t25317: F, t25391: F, t25392: F, t27265: F, t29636: F, t6048: F, t7048: F, t7053: F, t7070: F, t7071: F, t886: F, t93334: F, t93335: F, t93339: F, t99456: F, t27341: F, t99463: F, t99466: F, t14495: F, t27199: F, t27287: F, t29659: F, t4533: F, t7067: F, t7759: F, t93372: F, t99414: F, t99460: F, t99465: F, t99468: F, t99472: F, t99475: F, t99480: F, t99481: F, t25383: F, t29644: F, t7769: F, t7770: F, t93378: F, t93382: F, t93384: F, t93391: F, t99303: F, t99485: F, t99487: F, t99491: F, t99493: F, t99496: F, t99502: F, t99520: F, t99522: F, t105958: F, t105969: F, t106116: F, t106134: F, t106164: F, t106190: F, t106215: F, t106245: F, t106284: F, t106313: F, t106342: F, t106382: F, t106403: F, t892: F, t198: F, t205: F, t7782: F, t25207: F, t77441: F, t1544: F, t580: F, t98646: F, t25206: F, t105898: F, t105902: F, t105906: F, t105909: F, t105919: F, t105924: F, t105930: F, t1940: F, t1963: F, t2403: F, t25440: F, t27160: F, t29591: F, t29602: F, t29716: F, t30: F, t4541: F, t7087: F, t18435: F, t27159: F, t29598: F, t890: F, t27383: F, t18838: F, t2411: F, t29704: F, t18875: F, t98658: F, t92790: F, t775: F, t2255: F, t7783: F, t77425: F, t1468: F, t27158: F, t27166: F, t27173: F, t27364: F, t27368: F, t27391: F, t29705: F, t605: F, t7091: F, t7092: F, t7787: F, t98637: F, t99555: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t106404, t106407, t106410, t106423, t106430, t106431) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2137::<F>(t18657, t1955, t18797, t25399, t1579, t231, t4423, t1580, t27194, t689, t29690, t25411);
        let t106441 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2138::<F>(t106430, t25431, t106404, t106407, t106410, t106423, t106431, t1579, t18313, t18663, t1959, t25317, t25391, t25392, t27265, t29636, t6048, t7048, t7053, t7070, t7071, t886, t93334, t93335, t93339, t99456);
        let t106461 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2139::<F>(t27341, t99463, t99466, t14495, t25391, t27199, t27287, t29659, t4533, t7067, t7070, t7071, t7759, t93372, t99414, t99460, t99465, t99468, t99472, t99475, t99480, t99481);
        let t106477 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2140::<F>(t25317, t25383, t29644, t4533, t7070, t7769, t7770, t93378, t93382, t93384, t93391, t99303, t99485, t99487, t99491, t99493, t99496, t99502, t99520, t99522);
        let t106481 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2141::<F>(t105958, t105969, t106116, t106134, t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403, t106441, t106461, t106477);
        let (t106482, t106487, t106496, t106497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2142::<F>(t106481, t892, t198, t205, t7782, t25207, t77441, t1544, t580, t98646, t25206, t105898, t105902, t105906, t105909, t105919, t105924, t105930, t1940, t1963, t2403, t25440, t27160, t29591, t29602, t29716, t30, t4541, t7087);
        let (t106498, t106501, t106502, t106510, t106516, t106520, t106528) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2143::<F>(t18435, t27159, t29598, t890, t27383, t18838, t30, t2411, t29704, t18875, t98658, t92790);
        let (t106533, t106539, t106543) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2144::<F>(t29598, t775, t25207, t1940, t2255, t7783, t77425, t106498, t106502, t106510, t106516, t106520, t106528, t1468, t2403, t25206, t27158, t27166, t27173, t27364, t27368, t27391, t29705, t605, t7091, t7092, t7787, t98637, t99555);
    (t106481, t106482, t106487, t106496, t106497, t106501, t106516, t106533, t106539, t106543)
}
