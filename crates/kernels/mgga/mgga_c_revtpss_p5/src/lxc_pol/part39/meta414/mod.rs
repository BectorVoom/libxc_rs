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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta414<F: Float>(t31377: F, t571: F, t1464: F, t8372: F, t2178: F, t2371: F, t670: F, t8273: F, t31027: F, t31271: F, t116929: F, t8358: F, t31032: F, t31280: F, t46089: F, t655: F, t31288: F, t116926: F, t8355: F, t31264: F, t116938: F, t116957: F, t13509: F, t1504: F, t1513: F, t2: F, t31039: F, t31054: F, t31287: F, t4287: F, t8258: F, t8259: F, t8267: F, t31277: F, t31284: F, t116912: F, t31261: F, t10208: F, t69: F, t96: F, t100: F, t2339: F, t31268: F, t101460: F, t101463: F, t10199: F, t116942: F, t2174: F, t2256: F, t2366: F, t31035: F, t31043: F, t31058: F, t31283: F, t4269: F, t8268: F, t665: F, t101457: F, t116919: F, t116946: F, t2340: F, t2350: F, t28036: F, t31267: F, t31276: F, t658: F, t116913: F, t116915: F, t116917: F, t116927: F, t116930: F, t116932: F, t116934: F, t116936: F, t116968: F, t116969: F, t116971: F, t116995: F, t114: F, t101522: F, t1312: F, t13435: F, t18153: F, t18163: F, t1911: F, t2181: F, t27123: F, t28219: F, t31066: F, t31067: F, t31070: F, t31084: F, t31309: F, t31318: F, t31324: F, t4151: F, t4248: F, t4254: F, t508: F, t5523: F, t651: F, t7889: F, t8278: F, t8280: F, t8362: F, t8363: F, t8369: F, t98484: F, t98487: F, t10416: F, t13426: F, t1453: F, t18227: F, t1843: F, t2179: F, t2322: F, t27126: F, t31248: F, t31292: F, t31314: F, t49686: F, t75485: F, t75667: F, t8254: F, t98535: F, t1310: F, t13440: F, t31016: F, t31073: F, t31299: F, t31320: F, t3813: F, t7732: F, t8274: F, t8353: F, t14310: F, t31013: F, t31293: F, t5517: F, t569: F, t5787: F, t8367: F, t1518: F, t18190: F, t18204: F, t18208: F, t18211: F, t18214: F, t1916: F, t1918: F, t2187: F, t2189: F, t31100: F, t31118: F, t31121: F, t31358: F, t4162: F, t4165: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t8289: F, t8296: F, t8299: F, t8377: F, param_d: F, t116: F, t117: F, t117103: F, t13514: F, t1459: F, t1461: F, t2327: F, t31114: F, t31117: F, t31124: F, t31340: F, t31359: F, t31362: F, t31365: F, t31370: F, t31371: F, t31374: F, t4158: F, t5802: F, t8295: F, t8383: F, t8386: F) -> (F, F, F, F, F) {
        let (t117369, t117374, t117381, t117385, t117450, t117457) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1492::<F>(t31377, t571, t1464, t8372, t2178, t2371, t670, t8273, t31027, t31271, t116929, t8358);
        let t117477 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1493::<F>(t31032, t31280, t46089, t655, t31288, t116926, t8355, t31027, t31264, t116938, t116957, t117450, t117457, t13509, t1504, t1513, t2, t31039, t31054, t31287, t4287, t8258, t8259, t8267);
        let (t117482, t117484, t117497, t117499, t117500, t117505) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1494::<F>(t31027, t31277, t31032, t31284, t116912, t31261, t10208, t69, t96, t100, t1513, t2339);
        let t117517 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1495::<F>(t31027, t31268, t100, t101460, t101463, t10199, t116942, t117482, t117484, t117497, t117499, t117500, t117505, t1504, t2174, t2256, t2366, t31035, t31043, t31058, t31283, t4269, t8258, t8259, t8267, t8268);
        let t117560 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1496::<F>(t10199, t2339, t2, t665, t101457, t116919, t116946, t1504, t1513, t2256, t2340, t2350, t28036, t31035, t31039, t31054, t31058, t31267, t31276, t31287, t4287, t658, t8258, t8259, t8267, t8268);
        let t117572 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497::<F>(t116913, t116915, t116917, t116927, t116930, t116932, t116934, t116936, t116968, t116969, t116971, t116995);
        let (t117575, t117579) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1498::<F>(t114, t117477, t117517, t117560, t117572, t101522, t1312, t13435, t18153, t18163, t1911, t2178, t2181, t27123, t28219, t31066, t31067, t31070, t31084, t31309, t31318, t31324, t4151, t4248, t4254, t508, t5523, t651, t7889, t8278, t8280, t8362, t8363, t8369, t98484, t98487);
        let t117622 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1499::<F>(t10416, t1312, t13426, t13435, t1453, t18227, t1843, t2179, t2181, t2322, t27123, t27126, t31066, t31070, t31248, t31292, t31314, t31318, t4248, t4254, t49686, t5523, t651, t75485, t75667, t8254, t8278, t8280, t8363, t98535);
        let t117666 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1500::<F>(t10416, t1310, t13426, t13440, t18163, t18227, t2179, t2322, t27123, t31016, t31073, t31248, t31292, t31299, t31309, t31314, t31320, t31324, t3813, t4248, t4254, t651, t7732, t8254, t8274, t8280, t8353, t8362, t8369, t98484, t98487);
        let t117711 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1501::<F>(t10416, t117575, t1312, t13435, t13440, t14310, t18227, t2178, t2179, t2322, t27123, t27126, t31013, t31016, t31293, t31299, t31320, t4248, t5517, t5523, t569, t5787, t651, t75485, t7732, t8254, t8273, t8274, t8353, t8367);
        let (t117713, t117720) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1502::<F>(t117579, t117622, t117666, t117711, t117381, t117385, t1518, t18190, t18204, t18208, t18211, t18214, t1916, t1918, t2187, t2189, t31100, t31118, t31121, t31358, t4162, t4165, t4292, t572, t573, t5795, t5805, t8289, t8296, t8299, t8377, param_d);
        let t117765 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1503::<F>(t116, t31292, t117, t117103, t117575, t13514, t1459, t1461, t1518, t1916, t2327, t2371, t31114, t31117, t31124, t31340, t31359, t31362, t31365, t31370, t31371, t31374, t4158, t4292, t572, t5802, t670, t8289, t8295, t8362, t8383, t8386);
    (t117369, t117374, t117713, t117720, t117765)
}
