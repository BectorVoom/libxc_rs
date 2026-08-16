//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2004;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta633<F: Float>(t1081: F, t1649: F, t1877: F, t2057: F, t23789: F, t23813: F, t24191: F, t24335: F, t2522: F, t26563: F, t26740: F, t26744: F, t26756: F, t3231: F, t4314: F, t47645: F, t7114: F, t7649: F, t7845: F, t7871: F, t89859: F, t89862: F, t89865: F, t89868: F, t89874: F, t89896: F, t89904: F, t89954: F, t92319: F, t24339: F, t25905: F, t25921: F, t25930: F, t25934: F, t6841: F, t7110: F, t84797: F, t89850: F, t89888: F, t89892: F, t89911: F, t89917: F, t89978: F, t92356: F, t92359: F, t92362: F, t92364: F, t23781: F, t23796: F, t23810: F, t24344: F, t25901: F, t6848: F, t7656: F, t84791: F, t89837: F, t89840: F, t89846: F, t89872: F, t89907: F, t89931: F, t89941: F, t89982: F, t89993: F, t92276: F, t28: F, t265: F, t504: F, t93100: F, t12606: F, t1409: F, t2071: F, t2250: F, t24420: F, t26862: F, t3966: F, t52: F, t607: F, t7150: F, t7884: F, t93144: F, dens_threshold: F, rho1: F, zeta_threshold: F, t113: F, t12725: F, t12823: F, t1393: F, t1459: F, t1774: F, t1849: F, t1983: F, t2094: F, t22574: F, t23941: F, t24026: F, t24166: F, t24167: F, t24432: F, t24987: F, t26870: F, t26880: F, t26974: F, t27144: F, t27163: F, t27215: F, t3734: F, t4026: F, t4034: F, t510: F, t5161: F, t56198: F, t650: F, t6876: F, t6999: F, t7061: F, t7156: F, t7218: F, t7685: F, t7687: F, t7796: F, t83886: F, t84097: F, t92073: F, t93113: F, t27143: F, t532: F, t90459: F, t90468: F, t90470: F, t90472: F, t225: F, t27137: F, t27059: F, t2091: F, t40590: F, t1386: F, t16474: F, t24082: F, t26224: F, t5354: F, t80647: F, t80659: F, t80663: F, t80665: F, t80667: F, t80671: F, t90462: F, t90466: F, t90477: F, t90485: F, t90491: F, t90498: F) -> (F, F, F) {
        let t93181 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001::<F>(t1081, t1649, t1877, t2057, t23789, t23813, t24191, t24335, t2522, t26563, t26740, t26744, t26756, t3231, t4314, t47645, t7114, t7649, t7845, t7871, t89859, t89862, t89865, t89868, t89874, t89896, t89904, t89954, t92319);
        let t93211 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002::<F>(t1877, t2057, t24191, t24339, t2522, t25905, t25921, t25930, t25934, t26740, t26756, t6841, t7110, t7114, t84797, t89850, t89888, t89892, t89911, t89917, t89978, t92356, t92359, t92362, t92364);
        let t93246 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2003::<F>(t1877, t23781, t23796, t23810, t24191, t24344, t2522, t25901, t26744, t26756, t4314, t6848, t7110, t7114, t7656, t7845, t84791, t89837, t89840, t89846, t89872, t89907, t89931, t89941, t89982, t89993, t92276);
        let t93261 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2004::<F>(t28, t265, t504, t93100, t12606, t1409, t2071, t2250, t24420, t26862, t3966, t52, t607, t7150, t7884, t93144, t93181, t93211, t93246, dens_threshold, rho1, zeta_threshold);
        let t93275 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005::<F>(t113, t12725, t12823, t1393, t1459, t1774, t1849, t1983, t2094, t22574, t23941, t24026, t24166, t24167, t24432, t24987, t26870, t26880, t26974, t27144, t27163, t27215, t3734, t4026, t4034, t510, t5161, t56198, t650, t6876, t6999, t7061, t7156, t7218, t7685, t7687, t7796, t83886, t84097, t92073, t93113, t93261);
        let (t93286, t93332) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006::<F>(t27143, t532, t90459, t90468, t90470, t90472, t225, t27137, t27059, t2091, t40590, t1386, t16474, t24082, t26224, t5354, t80647, t80659, t80663, t80665, t80667, t80671, t90462, t90466, t90477, t90485, t90491, t90498);
    (t93275, t93286, t93332)
}
