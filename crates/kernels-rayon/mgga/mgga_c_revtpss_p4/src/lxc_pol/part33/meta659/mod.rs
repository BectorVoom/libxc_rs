//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta659 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2128;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2129;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2130;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2131;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2132;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2133;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2134;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2135;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta659(t6017: f64, t886: f64, t29668: f64, t689: f64, t25431: f64, t25411: f64, t14495: f64, t25391: f64, t25392: f64, t27189: f64, t27312: f64, t27349: f64, t27353: f64, t4487: f64, t93151: f64, t93158: f64, t93161: f64, t99186: f64, t99188: f64, t99202: f64, t99206: f64, t99334: f64, t1955: f64, t27212: f64, t5977: f64, t7048: f64, t18784: f64, t1949: f64, t231: f64, t25383: f64, t27199: f64, t27300: f64, t27358: f64, t29655: f64, t29691: f64, t62589: f64, t7070: f64, t7071: f64, t7076: f64, t93175: f64, t93177: f64, t99174: f64, t99212: f64, t99216: f64, t99219: f64, t99222: f64, t99228: f64, t99231: f64, t25317: f64, t27207: f64, t29636: f64, t29654: f64, t29682: f64, t29683: f64, t29695: f64, t836: f64, t92864: f64, t92917: f64, t93184: f64, t93192: f64, t93195: f64, t99234: f64, t99243: f64, t99245: f64, t99258: f64, t99261: f64, t27216: f64, t27279: f64, t27213: f64, t6022: f64, t29674: f64, t14587: f64, t18324: f64, t18615: f64, t25322: f64, t2718: f64, t27267: f64, t27357: f64, t6072: f64, t7053: f64, t7759: f64, t7766: f64, t93206: f64, t93207: f64, t93210: f64, t93224: f64, t99274: f64, t4481: f64, t99285: f64, t212: f64, t780: f64, t105944: f64, t1558: f64, t25416: f64, t2723: f64, t27265: f64, t27275: f64, t29610: f64, t29643: f64, t29669: f64, t62593: f64, t7079: f64, t7779: f64, t93118: f64, t93231: f64, t93242: f64, t99287: f64, t99297: f64, t7014: f64, t5978: f64, t4533: f64, t27292: f64, t27313: f64, t27350: f64, t62624: f64, t62637: f64, t93252: f64, t93272: f64, t93273: f64, t99191: f64, t99307: f64, t99313: f64, t99323: f64, t99342: f64, t6049: f64, t106128: f64, t25375: f64, t18805: f64, t93261: f64, t29675: f64, t4423: f64, t4534: f64, t6016: f64, t62604: f64, t62695: f64, t93276: f64, t93278: f64, t99344: f64, t99346: f64, t99351: f64, t213: f64, t6048: f64, t6071: f64, t106111: f64, t1579: f64, t1956: f64, t1957: f64, t233: f64, t27354: f64, t29611: f64, t29698: f64, t62628: f64, t7073: f64, t7083: f64, t887: f64, t93286: f64, t93349: f64, t99366: f64, t99375: f64, t99381: f64, t105945: f64, t7063: f64, t7060: f64, t29637: f64, t786: f64, t789: f64, t27317: f64, t27322: f64, t7775: f64, t93306: f64, t93324: f64, t99303: f64, t99391: f64, t99406: f64, t99412: f64, t99420: f64, t99423: f64, t99425: f64, t99435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t106164 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2128(t6017, t886, t29668, t689, t25431, t25411, t14495, t25391, t25392, t27189, t27312, t27349, t27353, t4487, t93151, t93158, t93161, t99186, t99188, t99202, t99206, t99334);
        let (t106172, t106178, t106190) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2129(t1955, t27212, t5977, t7048, t18784, t1949, t231, t25383, t27199, t27300, t27353, t27358, t29655, t29691, t62589, t7070, t7071, t7076, t93175, t93177, t99174, t99212, t99216, t99219, t99222, t99228, t99231);
        let t106215 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2130(t231, t25317, t25383, t25391, t27199, t27207, t29636, t29654, t29682, t29683, t29695, t7070, t7076, t836, t886, t92864, t92917, t93184, t93192, t93195, t99234, t99243, t99245, t99258, t99261);
        let t106245 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2131(t27216, t27279, t27213, t6022, t886, t29674, t689, t25431, t25411, t14587, t18324, t18615, t1949, t231, t25322, t25391, t2718, t27267, t27353, t27357, t6072, t7053, t7070, t7076, t7759, t7766, t93206, t93207, t93210, t93224, t99274);
        let (t106275, t106284) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2132(t4481, t99285, t212, t29636, t689, t780, t105944, t1955, t106178, t1558, t231, t25317, t25383, t25416, t2723, t27265, t27275, t27353, t27357, t29610, t29643, t29669, t62593, t7070, t7076, t7079, t7779, t886, t93118, t93231, t93242, t99287, t99297);
        let t106313 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2133(t6072, t689, t7014, t5978, t886, t1558, t231, t4533, t25391, t25392, t27199, t27292, t27313, t27350, t27353, t62624, t62637, t93252, t93272, t93273, t99191, t99307, t99313, t99323, t99342);
        let t106342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2134(t6049, t689, t7014, t106128, t25375, t18805, t93261, t231, t25383, t25392, t27189, t27353, t27357, t29675, t4423, t4534, t6016, t62604, t62695, t7048, t7070, t7076, t7759, t93276, t93278, t99344, t99346, t99351);
        let t106382 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2135(t213, t29636, t231, t6048, t836, t6071, t106111, t106172, t106275, t14587, t1579, t1956, t1957, t233, t25383, t25391, t25392, t27353, t27354, t27357, t29611, t29698, t62628, t7048, t7070, t7071, t7073, t7083, t887, t93286, t93349, t99366, t99375, t99381);
        let t106403 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2136(t105945, t7063, t7060, t29637, t786, t789, t27199, t27317, t27322, t7775, t93306, t93324, t99303, t99391, t99406, t99412, t99420, t99423, t99425, t99435);
    (t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403)
}
