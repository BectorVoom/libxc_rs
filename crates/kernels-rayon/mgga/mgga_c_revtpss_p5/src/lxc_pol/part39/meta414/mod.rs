//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta414 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1492;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1493;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1494;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1495;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1496;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1498;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1499;
use chunk8::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1500;
use chunk9::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1501;
use chunk10::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1502;
use chunk11::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta414(t31377: f64, t571: f64, t1464: f64, t8372: f64, t2178: f64, t2371: f64, t670: f64, t8273: f64, t31027: f64, t31271: f64, t116929: f64, t8358: f64, t31032: f64, t31280: f64, t46089: f64, t655: f64, t31288: f64, t116926: f64, t8355: f64, t31264: f64, t116938: f64, t116957: f64, t13509: f64, t1504: f64, t1513: f64, t2: f64, t31039: f64, t31054: f64, t31287: f64, t4287: f64, t8258: f64, t8259: f64, t8267: f64, t31277: f64, t31284: f64, t116912: f64, t31261: f64, t10208: f64, t69: f64, t96: f64, t100: f64, t2339: f64, t31268: f64, t101460: f64, t101463: f64, t10199: f64, t116942: f64, t2174: f64, t2256: f64, t2366: f64, t31035: f64, t31043: f64, t31058: f64, t31283: f64, t4269: f64, t8268: f64, t665: f64, t101457: f64, t116919: f64, t116946: f64, t2340: f64, t2350: f64, t28036: f64, t31267: f64, t31276: f64, t658: f64, t116913: f64, t116915: f64, t116917: f64, t116927: f64, t116930: f64, t116932: f64, t116934: f64, t116936: f64, t116968: f64, t116969: f64, t116971: f64, t116995: f64, t114: f64, t101522: f64, t1312: f64, t13435: f64, t18153: f64, t18163: f64, t1911: f64, t2181: f64, t27123: f64, t28219: f64, t31066: f64, t31067: f64, t31070: f64, t31084: f64, t31309: f64, t31318: f64, t31324: f64, t4151: f64, t4248: f64, t4254: f64, t508: f64, t5523: f64, t651: f64, t7889: f64, t8278: f64, t8280: f64, t8362: f64, t8363: f64, t8369: f64, t98484: f64, t98487: f64, t10416: f64, t13426: f64, t1453: f64, t18227: f64, t1843: f64, t2179: f64, t2322: f64, t27126: f64, t31248: f64, t31292: f64, t31314: f64, t49686: f64, t75485: f64, t75667: f64, t8254: f64, t98535: f64, t1310: f64, t13440: f64, t31016: f64, t31073: f64, t31299: f64, t31320: f64, t3813: f64, t7732: f64, t8274: f64, t8353: f64, t14310: f64, t31013: f64, t31293: f64, t5517: f64, t569: f64, t5787: f64, t8367: f64, t1518: f64, t18190: f64, t18204: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t1918: f64, t2187: f64, t2189: f64, t31100: f64, t31118: f64, t31121: f64, t31358: f64, t4162: f64, t4165: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t8289: f64, t8296: f64, t8299: f64, t8377: f64, param_d: f64, t116: f64, t117: f64, t117103: f64, t13514: f64, t1459: f64, t1461: f64, t2327: f64, t31114: f64, t31117: f64, t31124: f64, t31340: f64, t31359: f64, t31362: f64, t31365: f64, t31370: f64, t31371: f64, t31374: f64, t4158: f64, t5802: f64, t8295: f64, t8383: f64, t8386: f64) -> (f64, f64, f64, f64, f64) {
        let (t117369, t117374, t117381, t117385, t117450, t117457) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1492(t31377, t571, t1464, t8372, t2178, t2371, t670, t8273, t31027, t31271, t116929, t8358);
        let t117477 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1493(t31032, t31280, t46089, t655, t31288, t116926, t8355, t31027, t31264, t116938, t116957, t117450, t117457, t13509, t1504, t1513, t2, t31039, t31054, t31287, t4287, t8258, t8259, t8267);
        let (t117482, t117484, t117497, t117499, t117500, t117505) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1494(t31027, t31277, t31032, t31284, t116912, t31261, t10208, t69, t96, t100, t1513, t2339);
        let t117517 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1495(t31027, t31268, t100, t101460, t101463, t10199, t116942, t117482, t117484, t117497, t117499, t117500, t117505, t1504, t2174, t2256, t2366, t31035, t31043, t31058, t31283, t4269, t8258, t8259, t8267, t8268);
        let t117560 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1496(t10199, t2339, t2, t665, t101457, t116919, t116946, t1504, t1513, t2256, t2340, t2350, t28036, t31035, t31039, t31054, t31058, t31267, t31276, t31287, t4287, t658, t8258, t8259, t8267, t8268);
        let t117572 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497(t116913, t116915, t116917, t116927, t116930, t116932, t116934, t116936, t116968, t116969, t116971, t116995);
        let (t117575, t117579) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1498(t114, t117477, t117517, t117560, t117572, t101522, t1312, t13435, t18153, t18163, t1911, t2178, t2181, t27123, t28219, t31066, t31067, t31070, t31084, t31309, t31318, t31324, t4151, t4248, t4254, t508, t5523, t651, t7889, t8278, t8280, t8362, t8363, t8369, t98484, t98487);
        let t117622 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1499(t10416, t1312, t13426, t13435, t1453, t18227, t1843, t2179, t2181, t2322, t27123, t27126, t31066, t31070, t31248, t31292, t31314, t31318, t4248, t4254, t49686, t5523, t651, t75485, t75667, t8254, t8278, t8280, t8363, t98535);
        let t117666 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1500(t10416, t1310, t13426, t13440, t18163, t18227, t2179, t2322, t27123, t31016, t31073, t31248, t31292, t31299, t31309, t31314, t31320, t31324, t3813, t4248, t4254, t651, t7732, t8254, t8274, t8280, t8353, t8362, t8369, t98484, t98487);
        let t117711 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1501(t10416, t117575, t1312, t13435, t13440, t14310, t18227, t2178, t2179, t2322, t27123, t27126, t31013, t31016, t31293, t31299, t31320, t4248, t5517, t5523, t569, t5787, t651, t75485, t7732, t8254, t8273, t8274, t8353, t8367);
        let (t117713, t117720) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1502(t117579, t117622, t117666, t117711, t117381, t117385, t1518, t18190, t18204, t18208, t18211, t18214, t1916, t1918, t2187, t2189, t31100, t31118, t31121, t31358, t4162, t4165, t4292, t572, t573, t5795, t5805, t8289, t8296, t8299, t8377, param_d);
        let t117765 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1503(t116, t31292, t117, t117103, t117575, t13514, t1459, t1461, t1518, t1916, t2327, t2371, t31114, t31117, t31124, t31340, t31359, t31362, t31365, t31370, t31371, t31374, t4158, t4292, t572, t5802, t670, t8289, t8295, t8362, t8383, t8386);
    (t117369, t117374, t117713, t117720, t117765)
}
