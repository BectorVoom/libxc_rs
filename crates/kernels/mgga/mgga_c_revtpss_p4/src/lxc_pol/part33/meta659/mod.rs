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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta659<F: Float>(t6017: F, t886: F, t29668: F, t689: F, t25431: F, t25411: F, t14495: F, t25391: F, t25392: F, t27189: F, t27312: F, t27349: F, t27353: F, t4487: F, t93151: F, t93158: F, t93161: F, t99186: F, t99188: F, t99202: F, t99206: F, t99334: F, t1955: F, t27212: F, t5977: F, t7048: F, t18784: F, t1949: F, t231: F, t25383: F, t27199: F, t27300: F, t27358: F, t29655: F, t29691: F, t62589: F, t7070: F, t7071: F, t7076: F, t93175: F, t93177: F, t99174: F, t99212: F, t99216: F, t99219: F, t99222: F, t99228: F, t99231: F, t25317: F, t27207: F, t29636: F, t29654: F, t29682: F, t29683: F, t29695: F, t836: F, t92864: F, t92917: F, t93184: F, t93192: F, t93195: F, t99234: F, t99243: F, t99245: F, t99258: F, t99261: F, t27216: F, t27279: F, t27213: F, t6022: F, t29674: F, t14587: F, t18324: F, t18615: F, t25322: F, t2718: F, t27267: F, t27357: F, t6072: F, t7053: F, t7759: F, t7766: F, t93206: F, t93207: F, t93210: F, t93224: F, t99274: F, t4481: F, t99285: F, t212: F, t780: F, t105944: F, t1558: F, t25416: F, t2723: F, t27265: F, t27275: F, t29610: F, t29643: F, t29669: F, t62593: F, t7079: F, t7779: F, t93118: F, t93231: F, t93242: F, t99287: F, t99297: F, t7014: F, t5978: F, t4533: F, t27292: F, t27313: F, t27350: F, t62624: F, t62637: F, t93252: F, t93272: F, t93273: F, t99191: F, t99307: F, t99313: F, t99323: F, t99342: F, t6049: F, t106128: F, t25375: F, t18805: F, t93261: F, t29675: F, t4423: F, t4534: F, t6016: F, t62604: F, t62695: F, t93276: F, t93278: F, t99344: F, t99346: F, t99351: F, t213: F, t6048: F, t6071: F, t106111: F, t1579: F, t1956: F, t1957: F, t233: F, t27354: F, t29611: F, t29698: F, t62628: F, t7073: F, t7083: F, t887: F, t93286: F, t93349: F, t99366: F, t99375: F, t99381: F, t105945: F, t7063: F, t7060: F, t29637: F, t786: F, t789: F, t27317: F, t27322: F, t7775: F, t93306: F, t93324: F, t99303: F, t99391: F, t99406: F, t99412: F, t99420: F, t99423: F, t99425: F, t99435: F) -> (F, F, F, F, F, F, F, F, F) {
        let t106164 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2128::<F>(t6017, t886, t29668, t689, t25431, t25411, t14495, t25391, t25392, t27189, t27312, t27349, t27353, t4487, t93151, t93158, t93161, t99186, t99188, t99202, t99206, t99334);
        let (t106172, t106178, t106190) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2129::<F>(t1955, t27212, t5977, t7048, t18784, t1949, t231, t25383, t27199, t27300, t27353, t27358, t29655, t29691, t62589, t7070, t7071, t7076, t93175, t93177, t99174, t99212, t99216, t99219, t99222, t99228, t99231);
        let t106215 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2130::<F>(t231, t25317, t25383, t25391, t27199, t27207, t29636, t29654, t29682, t29683, t29695, t7070, t7076, t836, t886, t92864, t92917, t93184, t93192, t93195, t99234, t99243, t99245, t99258, t99261);
        let t106245 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2131::<F>(t27216, t27279, t27213, t6022, t886, t29674, t689, t25431, t25411, t14587, t18324, t18615, t1949, t231, t25322, t25391, t2718, t27267, t27353, t27357, t6072, t7053, t7070, t7076, t7759, t7766, t93206, t93207, t93210, t93224, t99274);
        let (t106275, t106284) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2132::<F>(t4481, t99285, t212, t29636, t689, t780, t105944, t1955, t106178, t1558, t231, t25317, t25383, t25416, t2723, t27265, t27275, t27353, t27357, t29610, t29643, t29669, t62593, t7070, t7076, t7079, t7779, t886, t93118, t93231, t93242, t99287, t99297);
        let t106313 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2133::<F>(t6072, t689, t7014, t5978, t886, t1558, t231, t4533, t25391, t25392, t27199, t27292, t27313, t27350, t27353, t62624, t62637, t93252, t93272, t93273, t99191, t99307, t99313, t99323, t99342);
        let t106342 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2134::<F>(t6049, t689, t7014, t106128, t25375, t18805, t93261, t231, t25383, t25392, t27189, t27353, t27357, t29675, t4423, t4534, t6016, t62604, t62695, t7048, t7070, t7076, t7759, t93276, t93278, t99344, t99346, t99351);
        let t106382 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2135::<F>(t213, t29636, t231, t6048, t836, t6071, t106111, t106172, t106275, t14587, t1579, t1956, t1957, t233, t25383, t25391, t25392, t27353, t27354, t27357, t29611, t29698, t62628, t7048, t7070, t7071, t7073, t7083, t887, t93286, t93349, t99366, t99375, t99381);
        let t106403 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2136::<F>(t105945, t7063, t7060, t29637, t786, t789, t27199, t27317, t27322, t7775, t93306, t93324, t99303, t99391, t99406, t99412, t99420, t99423, t99425, t99435);
    (t106164, t106190, t106215, t106245, t106284, t106313, t106342, t106382, t106403)
}
