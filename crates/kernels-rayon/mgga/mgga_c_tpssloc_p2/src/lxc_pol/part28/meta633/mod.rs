//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2004;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta633(t1081: f64, t1649: f64, t1877: f64, t2057: f64, t23789: f64, t23813: f64, t24191: f64, t24335: f64, t2522: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t3231: f64, t4314: f64, t47645: f64, t7114: f64, t7649: f64, t7845: f64, t7871: f64, t89859: f64, t89862: f64, t89865: f64, t89868: f64, t89874: f64, t89896: f64, t89904: f64, t89954: f64, t92319: f64, t24339: f64, t25905: f64, t25921: f64, t25930: f64, t25934: f64, t6841: f64, t7110: f64, t84797: f64, t89850: f64, t89888: f64, t89892: f64, t89911: f64, t89917: f64, t89978: f64, t92356: f64, t92359: f64, t92362: f64, t92364: f64, t23781: f64, t23796: f64, t23810: f64, t24344: f64, t25901: f64, t6848: f64, t7656: f64, t84791: f64, t89837: f64, t89840: f64, t89846: f64, t89872: f64, t89907: f64, t89931: f64, t89941: f64, t89982: f64, t89993: f64, t92276: f64, t28: f64, t265: f64, t504: f64, t93100: f64, t12606: f64, t1409: f64, t2071: f64, t2250: f64, t24420: f64, t26862: f64, t3966: f64, t52: f64, t607: f64, t7150: f64, t7884: f64, t93144: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t113: f64, t12725: f64, t12823: f64, t1393: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2094: f64, t22574: f64, t23941: f64, t24026: f64, t24166: f64, t24167: f64, t24432: f64, t24987: f64, t26870: f64, t26880: f64, t26974: f64, t27144: f64, t27163: f64, t27215: f64, t3734: f64, t4026: f64, t4034: f64, t510: f64, t5161: f64, t56198: f64, t650: f64, t6876: f64, t6999: f64, t7061: f64, t7156: f64, t7218: f64, t7685: f64, t7687: f64, t7796: f64, t83886: f64, t84097: f64, t92073: f64, t93113: f64, t27143: f64, t532: f64, t90459: f64, t90468: f64, t90470: f64, t90472: f64, t225: f64, t27137: f64, t27059: f64, t2091: f64, t40590: f64, t1386: f64, t16474: f64, t24082: f64, t26224: f64, t5354: f64, t80647: f64, t80659: f64, t80663: f64, t80665: f64, t80667: f64, t80671: f64, t90462: f64, t90466: f64, t90477: f64, t90485: f64, t90491: f64, t90498: f64) -> (f64, f64, f64) {
        let t93181 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001(t1081, t1649, t1877, t2057, t23789, t23813, t24191, t24335, t2522, t26563, t26740, t26744, t26756, t3231, t4314, t47645, t7114, t7649, t7845, t7871, t89859, t89862, t89865, t89868, t89874, t89896, t89904, t89954, t92319);
        let t93211 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002(t1877, t2057, t24191, t24339, t2522, t25905, t25921, t25930, t25934, t26740, t26756, t6841, t7110, t7114, t84797, t89850, t89888, t89892, t89911, t89917, t89978, t92356, t92359, t92362, t92364);
        let t93246 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003(t1877, t23781, t23796, t23810, t24191, t24344, t2522, t25901, t26744, t26756, t4314, t6848, t7110, t7114, t7656, t7845, t84791, t89837, t89840, t89846, t89872, t89907, t89931, t89941, t89982, t89993, t92276);
        let t93261 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2004(t28, t265, t504, t93100, t12606, t1409, t2071, t2250, t24420, t26862, t3966, t52, t607, t7150, t7884, t93144, t93181, t93211, t93246, dens_threshold, rho1, zeta_threshold);
        let t93275 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005(t113, t12725, t12823, t1393, t1459, t1774, t1849, t1983, t2094, t22574, t23941, t24026, t24166, t24167, t24432, t24987, t26870, t26880, t26974, t27144, t27163, t27215, t3734, t4026, t4034, t510, t5161, t56198, t650, t6876, t6999, t7061, t7156, t7218, t7685, t7687, t7796, t83886, t84097, t92073, t93113, t93261);
        let (t93286, t93332) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006(t27143, t532, t90459, t90468, t90470, t90472, t225, t27137, t27059, t2091, t40590, t1386, t16474, t24082, t26224, t5354, t80647, t80659, t80663, t80665, t80667, t80671, t90462, t90466, t90477, t90485, t90491, t90498);
    (t93275, t93286, t93332)
}
