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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta647(t4481: f64, t99285: f64, t212: f64, t29636: f64, t689: f64, t780: f64, t105944: f64, t1955: f64, t106178: f64, t1558: f64, t231: f64, t25317: f64, t25383: f64, t25416: f64, t2723: f64, t27265: f64, t27275: f64, t27353: f64, t27357: f64, t29610: f64, t29643: f64, t29669: f64, t62593: f64, t7070: f64, t7076: f64, t7079: f64, t7779: f64, t886: f64, t93118: f64, t93231: f64, t93242: f64, t99287: f64, t99297: f64, t6072: f64, t7014: f64, t5978: f64, t4533: f64, t25391: f64, t25392: f64, t27199: f64, t27292: f64, t27313: f64, t27350: f64, t62624: f64, t62637: f64, t93252: f64, t93272: f64, t93273: f64, t99191: f64, t99307: f64, t99313: f64, t99323: f64, t99342: f64, t6049: f64, t106128: f64, t25375: f64, t18805: f64, t93261: f64, t27189: f64, t29675: f64, t4423: f64, t4534: f64, t6016: f64, t62604: f64, t62695: f64, t7048: f64, t7759: f64, t93276: f64, t93278: f64, t99344: f64, t99346: f64, t99351: f64, t213: f64, t6048: f64, t836: f64, t6071: f64, t106111: f64, t106172: f64, t14587: f64, t1579: f64, t1956: f64, t1957: f64, t233: f64, t27354: f64, t29611: f64, t29698: f64, t62628: f64, t7071: f64, t7073: f64, t7083: f64, t887: f64, t93286: f64, t93349: f64, t99366: f64, t99375: f64, t99381: f64, t105945: f64, t7063: f64, t7060: f64, t29637: f64, t786: f64, t789: f64, t27317: f64, t27322: f64, t7775: f64, t93306: f64, t93324: f64, t99303: f64, t99391: f64, t99406: f64, t99412: f64, t99420: f64, t99423: f64, t99425: f64, t99435: f64, t18657: f64, t18797: f64, t25399: f64, t1580: f64, t27194: f64, t29690: f64, t25411: f64, t25431: f64, t18313: f64, t18663: f64, t1959: f64, t7053: f64, t93334: f64, t93335: f64, t93339: f64, t99456: f64, t27341: f64, t99463: f64, t99466: f64, t14495: f64, t27287: f64, t29659: f64, t7067: f64, t93372: f64, t99414: f64, t99460: f64, t99465: f64, t99468: f64, t99472: f64, t99475: f64, t99480: f64, t99481: f64, t29644: f64, t7769: f64, t7770: f64, t93378: f64, t93382: f64, t93384: f64, t93391: f64, t99485: f64, t99487: f64, t99491: f64, t99493: f64, t99496: f64, t99502: f64, t99520: f64, t99522: f64, t105958: f64, t105969: f64, t106116: f64, t106134: f64, t106164: f64, t106190: f64, t106215: f64, t106245: f64, t892: f64, t198: f64, t205: f64, t7782: f64, t25207: f64, t77441: f64, t1544: f64, t580: f64, t98646: f64, t25206: f64, t105898: f64, t105902: f64, t105906: f64, t105909: f64, t105919: f64, t105924: f64, t105930: f64, t1940: f64, t1963: f64, t2403: f64, t25440: f64, t27160: f64, t29591: f64, t29602: f64, t29716: f64, t30: f64, t4541: f64, t7087: f64, t18435: f64, t27159: f64, t29598: f64, t890: f64, t27383: f64, t18838: f64, t2411: f64, t29704: f64, t18875: f64, t98658: f64, t92790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106275, t106284) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2124(t4481, t99285, t212, t29636, t689, t780, t105944, t1955, t106178, t1558, t231, t25317, t25383, t25416, t2723, t27265, t27275, t27353, t27357, t29610, t29643, t29669, t62593, t7070, t7076, t7079, t7779, t886, t93118, t93231, t93242, t99287, t99297);
        let t106313 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2125(t6072, t689, t7014, t5978, t886, t1558, t231, t4533, t25391, t25392, t27199, t27292, t27313, t27350, t27353, t62624, t62637, t93252, t93272, t93273, t99191, t99307, t99313, t99323, t99342);
        let t106342 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2126(t6049, t689, t7014, t106128, t25375, t18805, t93261, t231, t25383, t25392, t27189, t27353, t27357, t29675, t4423, t4534, t6016, t62604, t62695, t7048, t7070, t7076, t7759, t93276, t93278, t99344, t99346, t99351);
        let t106382 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2127(t213, t29636, t231, t6048, t836, t6071, t106111, t106172, t106275, t14587, t1579, t1956, t1957, t233, t25383, t25391, t25392, t27353, t27354, t27357, t29611, t29698, t62628, t7048, t7070, t7071, t7073, t7083, t887, t93286, t93349, t99366, t99375, t99381);
        let t106403 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2128(t105945, t7063, t7060, t29637, t786, t789, t27199, t27317, t27322, t7775, t93306, t93324, t99303, t99391, t99406, t99412, t99420, t99423, t99425, t99435);
        let (t106404, t106407, t106410, t106423, t106430, t106431) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2129(t18657, t1955, t18797, t25399, t1579, t231, t4423, t1580, t27194, t689, t29690, t25411);
        let t106441 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2130(t106430, t25431, t106404, t106407, t106410, t106423, t106431, t1579, t18313, t18663, t1959, t25317, t25391, t25392, t27265, t29636, t6048, t7048, t7053, t7070, t7071, t886, t93334, t93335, t93339, t99456);
        let t106461 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2131(t27341, t99463, t99466, t14495, t25391, t27199, t27287, t29659, t4533, t7067, t7070, t7071, t7759, t93372, t99414, t99460, t99465, t99468, t99472, t99475, t99480, t99481);
        let t106477 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2132(t25317, t25383, t29644, t4533, t7070, t7769, t7770, t93378, t93382, t93384, t93391, t99303, t99485, t99487, t99491, t99493, t99496, t99502, t99520, t99522);
        let t106481 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2133(t105958, t105969, t106116, t106134, t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403, t106441, t106461, t106477);
        let (t106482, t106487, t106496, t106497) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2134(t106481, t892, t198, t205, t7782, t25207, t77441, t1544, t580, t98646, t25206, t105898, t105902, t105906, t105909, t105919, t105924, t105930, t1940, t1963, t2403, t25440, t27160, t29591, t29602, t29716, t30, t4541, t7087);
        let (t106498, t106501, t106502, t106510, t106516, t106520, t106528) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2135(t18435, t27159, t29598, t890, t27383, t18838, t30, t2411, t29704, t18875, t98658, t92790);
    (t106481, t106482, t106487, t106496, t106497, t106498, t106501, t106502, t106510, t106516, t106520, t106528)
}
