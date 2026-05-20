//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta647 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2124;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2125;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2126;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2127;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2128;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2129;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2130;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2131;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2132;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2133;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2134;
use chunk11::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta647<F: Float>(t4481: F, t99285: F, t212: F, t29636: F, t689: F, t780: F, t105944: F, t1955: F, t106178: F, t1558: F, t231: F, t25317: F, t25383: F, t25416: F, t2723: F, t27265: F, t27275: F, t27353: F, t27357: F, t29610: F, t29643: F, t29669: F, t62593: F, t7070: F, t7076: F, t7079: F, t7779: F, t886: F, t93118: F, t93231: F, t93242: F, t99287: F, t99297: F, t6072: F, t7014: F, t5978: F, t4533: F, t25391: F, t25392: F, t27199: F, t27292: F, t27313: F, t27350: F, t62624: F, t62637: F, t93252: F, t93272: F, t93273: F, t99191: F, t99307: F, t99313: F, t99323: F, t99342: F, t6049: F, t106128: F, t25375: F, t18805: F, t93261: F, t27189: F, t29675: F, t4423: F, t4534: F, t6016: F, t62604: F, t62695: F, t7048: F, t7759: F, t93276: F, t93278: F, t99344: F, t99346: F, t99351: F, t213: F, t6048: F, t836: F, t6071: F, t106111: F, t106172: F, t14587: F, t1579: F, t1956: F, t1957: F, t233: F, t27354: F, t29611: F, t29698: F, t62628: F, t7071: F, t7073: F, t7083: F, t887: F, t93286: F, t93349: F, t99366: F, t99375: F, t99381: F, t105945: F, t7063: F, t7060: F, t29637: F, t786: F, t789: F, t27317: F, t27322: F, t7775: F, t93306: F, t93324: F, t99303: F, t99391: F, t99406: F, t99412: F, t99420: F, t99423: F, t99425: F, t99435: F, t18657: F, t18797: F, t25399: F, t1580: F, t27194: F, t29690: F, t25411: F, t25431: F, t18313: F, t18663: F, t1959: F, t7053: F, t93334: F, t93335: F, t93339: F, t99456: F, t27341: F, t99463: F, t99466: F, t14495: F, t27287: F, t29659: F, t7067: F, t93372: F, t99414: F, t99460: F, t99465: F, t99468: F, t99472: F, t99475: F, t99480: F, t99481: F, t29644: F, t7769: F, t7770: F, t93378: F, t93382: F, t93384: F, t93391: F, t99485: F, t99487: F, t99491: F, t99493: F, t99496: F, t99502: F, t99520: F, t99522: F, t105958: F, t105969: F, t106116: F, t106134: F, t106164: F, t106190: F, t106215: F, t106245: F, t892: F, t198: F, t205: F, t7782: F, t25207: F, t77441: F, t1544: F, t580: F, t98646: F, t25206: F, t105898: F, t105902: F, t105906: F, t105909: F, t105919: F, t105924: F, t105930: F, t1940: F, t1963: F, t2403: F, t25440: F, t27160: F, t29591: F, t29602: F, t29716: F, t30: F, t4541: F, t7087: F, t18435: F, t27159: F, t29598: F, t890: F, t27383: F, t18838: F, t2411: F, t29704: F, t18875: F, t98658: F, t92790: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t106275, t106284) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2124::<F>(t4481, t99285, t212, t29636, t689, t780, t105944, t1955, t106178, t1558, t231, t25317, t25383, t25416, t2723, t27265, t27275, t27353, t27357, t29610, t29643, t29669, t62593, t7070, t7076, t7079, t7779, t886, t93118, t93231, t93242, t99287, t99297);
        let t106313 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2125::<F>(t6072, t689, t7014, t5978, t886, t1558, t231, t4533, t25391, t25392, t27199, t27292, t27313, t27350, t27353, t62624, t62637, t93252, t93272, t93273, t99191, t99307, t99313, t99323, t99342);
        let t106342 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2126::<F>(t6049, t689, t7014, t106128, t25375, t18805, t93261, t231, t25383, t25392, t27189, t27353, t27357, t29675, t4423, t4534, t6016, t62604, t62695, t7048, t7070, t7076, t7759, t93276, t93278, t99344, t99346, t99351);
        let t106382 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2127::<F>(t213, t29636, t231, t6048, t836, t6071, t106111, t106172, t106275, t14587, t1579, t1956, t1957, t233, t25383, t25391, t25392, t27353, t27354, t27357, t29611, t29698, t62628, t7048, t7070, t7071, t7073, t7083, t887, t93286, t93349, t99366, t99375, t99381);
        let t106403 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2128::<F>(t105945, t7063, t7060, t29637, t786, t789, t27199, t27317, t27322, t7775, t93306, t93324, t99303, t99391, t99406, t99412, t99420, t99423, t99425, t99435);
        let (t106404, t106407, t106410, t106423, t106430, t106431) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2129::<F>(t18657, t1955, t18797, t25399, t1579, t231, t4423, t1580, t27194, t689, t29690, t25411);
        let t106441 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2130::<F>(t106430, t25431, t106404, t106407, t106410, t106423, t106431, t1579, t18313, t18663, t1959, t25317, t25391, t25392, t27265, t29636, t6048, t7048, t7053, t7070, t7071, t886, t93334, t93335, t93339, t99456);
        let t106461 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2131::<F>(t27341, t99463, t99466, t14495, t25391, t27199, t27287, t29659, t4533, t7067, t7070, t7071, t7759, t93372, t99414, t99460, t99465, t99468, t99472, t99475, t99480, t99481);
        let t106477 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2132::<F>(t25317, t25383, t29644, t4533, t7070, t7769, t7770, t93378, t93382, t93384, t93391, t99303, t99485, t99487, t99491, t99493, t99496, t99502, t99520, t99522);
        let t106481 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2133::<F>(t105958, t105969, t106116, t106134, t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403, t106441, t106461, t106477);
        let (t106482, t106487, t106496, t106497) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2134::<F>(t106481, t892, t198, t205, t7782, t25207, t77441, t1544, t580, t98646, t25206, t105898, t105902, t105906, t105909, t105919, t105924, t105930, t1940, t1963, t2403, t25440, t27160, t29591, t29602, t29716, t30, t4541, t7087);
        let (t106498, t106501, t106502, t106510, t106516, t106520, t106528) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2135::<F>(t18435, t27159, t29598, t890, t27383, t18838, t30, t2411, t29704, t18875, t98658, t92790);
    (t106481, t106482, t106487, t106496, t106497, t106498, t106501, t106502, t106510, t106516, t106520, t106528)
}
