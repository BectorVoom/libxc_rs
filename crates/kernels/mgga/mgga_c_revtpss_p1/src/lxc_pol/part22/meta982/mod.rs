//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta982 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3323;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3324;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3325;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3326;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3327;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3328;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3329;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3330;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3331;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta982<F: Float>(t2439: F, t2440: F, t6072: F, t2444: F, t689: F, t11008: F, t14978: F, t1579: F, t1580: F, t18800: F, t2770: F, t2771: F, t2829: F, t51233: F, t51237: F, t51239: F, t51241: F, t51246: F, t51251: F, t51256: F, t51259: F, t51262: F, t51264: F, t51272: F, t6071: F, t62549: F, t62572: F, t62611: F, t62655: F, t62679: F, t62705: F, t62733: F, t62754: F, t62792: F, t62825: F, t62856: F, t62887: F, t62912: F, t62945: F, t62973: F, t63002: F, t63024: F, t63041: F, t865: F, t868: F, t15003: F, t51258: F, t18784: F, t2465: F, t686: F, t72: F, t4481: F, t51276: F, t6042: F, t786: F, t867: F, t2467: F, t15011: F, t15030: F, t2828: F, t41095: F, t41098: F, t41102: F, t41105: F, t4474: F, t4534: F, t51268: F, t51277: F, t51726: F, t51729: F, t51731: F, t51733: F, t51739: F, t51741: F, t14480: F, t252: F, t2782: F, t4533: F, t14991: F, t50208: F, t14485: F, t14987: F, t18657: F, t213: F, t14983: F, t10513: F, t15038: F, t18313: F, t18663: F, t18785: F, t2765: F, t41115: F, t41118: F, t41125: F, t51746: F, t51750: F, t51756: F, t51759: F, t6048: F, t6049: F, t886: F, t887: F, t14353: F, t14468: F, t18850: F, t198: F, t207: F, t2403: F, t2430: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t4343: F, t4546: F, t61358: F, t61387: F, t61429: F, t62269: F, t62270: F, t62273: F, t62518: F, t62545: F, t892: F, t18392: F, t262: F, t11084: F, t18860: F, t4541: F, t51780: F, t5966: F, t5970: F, t62275: F, t62277: F, t62279: F, t62283: F, t62285: F, t62286: F, t62290: F, t62293: F, t62296: F, t775: F, t18838: F, t2411: F, t4537: F, t890: F, t14436: F, t18256: F, t1940: F, t50080: F, t62297: F, t62298: F, t62299: F, t62300: F, t62301: F, t62303: F, t62304: F, t62305: F, t62306: F, t1544: F, t14365: F, t1583: F, t18865: F, t205: F, t2404: F, t2408: F, t2832: F, t40076: F, t40079: F, t40194: F, t40198: F, t41154: F, t6079: F, t61519: F, t62307: F, t62308: F, t62311: F, t62312: F, t765: F, t61030: F, t61106: F, t61125: F, t61146: F, t61163: F, t61174: F, t61192: F, t61210: F, t61230: F, t61262: F, t61291: F, t61318: F, t2: F, t4560: F, t580: F, t1587: F, t18890: F, t22: F, t4595: F, t52505: F, t4636: F, t52219: F, t15101: F, t15380: F) -> (F, F, F, F, F, F, F) {
        let t63055 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3323::<F>(t2439, t2440, t6072, t2444, t689, t11008, t14978, t1579, t1580, t18800, t2770, t2771, t2829, t51233, t51237, t51239, t51241, t51246, t51251, t51256, t51259, t51262, t51264, t51272, t6071, t62549, t62572, t62611, t62655, t62679, t62705, t62733, t62754, t62792, t62825, t62856, t62887, t62912, t62945, t62973, t63002, t63024, t63041, t865, t868);
        let (t63058, t63062, t63064, t63085) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3324::<F>(t15003, t51258, t18784, t2465, t686, t72, t4481, t51276, t6042, t786, t867, t2467);
        let t63088 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3325::<F>(t15011, t15030, t2770, t2828, t41095, t41098, t41102, t41105, t4474, t4534, t51268, t51277, t51726, t51729, t51731, t51733, t51739, t51741, t6071, t63058, t63062, t63064, t63085, t865);
        let t63129 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3326::<F>(t14480, t252, t2782, t4533, t14991, t50208, t14485, t14987, t18657, t213, t14983, t10513, t11008, t15038, t18313, t18663, t18784, t18785, t2765, t2770, t2828, t41115, t41118, t41125, t4474, t51746, t51750, t51756, t51759, t6048, t6049, t6072, t865, t886, t887);
        let t63145 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3327::<F>(t14353, t14468, t18850, t198, t207, t2403, t2430, t40067, t40072, t40167, t40171, t40184, t4343, t4546, t61358, t61387, t61429, t62269, t62270, t62273, t62518, t62545, t63055, t63088, t63129, t892);
        let t63158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3328::<F>(t18392, t262, t11084, t18860, t2430, t4541, t51780, t5966, t5970, t62275, t62277, t62279, t62283, t62285, t62286, t62290, t62293, t62296, t775);
        let t63170 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3329::<F>(t18838, t2411, t4537, t890, t14436, t18256, t1940, t50080, t62297, t62298, t62299, t62300, t62301, t62303, t62304, t62305, t62306);
        let t63189 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3330::<F>(t1544, t2411, t14365, t1583, t18392, t18865, t1940, t198, t205, t2403, t2404, t2408, t2832, t40076, t40079, t40194, t40198, t41154, t6079, t61519, t62307, t62308, t62311, t62312, t765);
        let t63193 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3331::<F>(t61030, t61106, t61125, t61146, t61163, t61174, t61192, t61210, t61230, t61262, t61291, t61318, t63145, t63158, t63170, t63189);
        let (t63202, t63204, t63206, t63212, t63214, t63216) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3332::<F>(t2, t4560, t580, t1587, t18890, t22, t4595, t52505, t4636, t52219, t15101, t15380);
    (t63193, t63202, t63204, t63206, t63212, t63214, t63216)
}
