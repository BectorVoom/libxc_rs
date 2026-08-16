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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta982(t2439: f64, t2440: f64, t6072: f64, t2444: f64, t689: f64, t11008: f64, t14978: f64, t1579: f64, t1580: f64, t18800: f64, t2770: f64, t2771: f64, t2829: f64, t51233: f64, t51237: f64, t51239: f64, t51241: f64, t51246: f64, t51251: f64, t51256: f64, t51259: f64, t51262: f64, t51264: f64, t51272: f64, t6071: f64, t62549: f64, t62572: f64, t62611: f64, t62655: f64, t62679: f64, t62705: f64, t62733: f64, t62754: f64, t62792: f64, t62825: f64, t62856: f64, t62887: f64, t62912: f64, t62945: f64, t62973: f64, t63002: f64, t63024: f64, t63041: f64, t865: f64, t868: f64, t15003: f64, t51258: f64, t18784: f64, t2465: f64, t686: f64, t72: f64, t4481: f64, t51276: f64, t6042: f64, t786: f64, t867: f64, t2467: f64, t15011: f64, t15030: f64, t2828: f64, t41095: f64, t41098: f64, t41102: f64, t41105: f64, t4474: f64, t4534: f64, t51268: f64, t51277: f64, t51726: f64, t51729: f64, t51731: f64, t51733: f64, t51739: f64, t51741: f64, t14480: f64, t252: f64, t2782: f64, t4533: f64, t14991: f64, t50208: f64, t14485: f64, t14987: f64, t18657: f64, t213: f64, t14983: f64, t10513: f64, t15038: f64, t18313: f64, t18663: f64, t18785: f64, t2765: f64, t41115: f64, t41118: f64, t41125: f64, t51746: f64, t51750: f64, t51756: f64, t51759: f64, t6048: f64, t6049: f64, t886: f64, t887: f64, t14353: f64, t14468: f64, t18850: f64, t198: f64, t207: f64, t2403: f64, t2430: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t4343: f64, t4546: f64, t61358: f64, t61387: f64, t61429: f64, t62269: f64, t62270: f64, t62273: f64, t62518: f64, t62545: f64, t892: f64, t18392: f64, t262: f64, t11084: f64, t18860: f64, t4541: f64, t51780: f64, t5966: f64, t5970: f64, t62275: f64, t62277: f64, t62279: f64, t62283: f64, t62285: f64, t62286: f64, t62290: f64, t62293: f64, t62296: f64, t775: f64, t18838: f64, t2411: f64, t4537: f64, t890: f64, t14436: f64, t18256: f64, t1940: f64, t50080: f64, t62297: f64, t62298: f64, t62299: f64, t62300: f64, t62301: f64, t62303: f64, t62304: f64, t62305: f64, t62306: f64, t1544: f64, t14365: f64, t1583: f64, t18865: f64, t205: f64, t2404: f64, t2408: f64, t2832: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t41154: f64, t6079: f64, t61519: f64, t62307: f64, t62308: f64, t62311: f64, t62312: f64, t765: f64, t61030: f64, t61106: f64, t61125: f64, t61146: f64, t61163: f64, t61174: f64, t61192: f64, t61210: f64, t61230: f64, t61262: f64, t61291: f64, t61318: f64, t2: f64, t4560: f64, t580: f64, t1587: f64, t18890: f64, t22: f64, t4595: f64, t52505: f64, t4636: f64, t52219: f64, t15101: f64, t15380: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t63055 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3323(t2439, t2440, t6072, t2444, t689, t11008, t14978, t1579, t1580, t18800, t2770, t2771, t2829, t51233, t51237, t51239, t51241, t51246, t51251, t51256, t51259, t51262, t51264, t51272, t6071, t62549, t62572, t62611, t62655, t62679, t62705, t62733, t62754, t62792, t62825, t62856, t62887, t62912, t62945, t62973, t63002, t63024, t63041, t865, t868);
        let (t63058, t63062, t63064, t63085) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3324(t15003, t51258, t18784, t2465, t686, t72, t4481, t51276, t6042, t786, t867, t2467);
        let t63088 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3325(t15011, t15030, t2770, t2828, t41095, t41098, t41102, t41105, t4474, t4534, t51268, t51277, t51726, t51729, t51731, t51733, t51739, t51741, t6071, t63058, t63062, t63064, t63085, t865);
        let t63129 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3326(t14480, t252, t2782, t4533, t14991, t50208, t14485, t14987, t18657, t213, t14983, t10513, t11008, t15038, t18313, t18663, t18784, t18785, t2765, t2770, t2828, t41115, t41118, t41125, t4474, t51746, t51750, t51756, t51759, t6048, t6049, t6072, t865, t886, t887);
        let t63145 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3327(t14353, t14468, t18850, t198, t207, t2403, t2430, t40067, t40072, t40167, t40171, t40184, t4343, t4546, t61358, t61387, t61429, t62269, t62270, t62273, t62518, t62545, t63055, t63088, t63129, t892);
        let t63158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3328(t18392, t262, t11084, t18860, t2430, t4541, t51780, t5966, t5970, t62275, t62277, t62279, t62283, t62285, t62286, t62290, t62293, t62296, t775);
        let t63170 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3329(t18838, t2411, t4537, t890, t14436, t18256, t1940, t50080, t62297, t62298, t62299, t62300, t62301, t62303, t62304, t62305, t62306);
        let t63189 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3330(t1544, t2411, t14365, t1583, t18392, t18865, t1940, t198, t205, t2403, t2404, t2408, t2832, t40076, t40079, t40194, t40198, t41154, t6079, t61519, t62307, t62308, t62311, t62312, t765);
        let t63193 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3331(t61030, t61106, t61125, t61146, t61163, t61174, t61192, t61210, t61230, t61262, t61291, t61318, t63145, t63158, t63170, t63189);
        let (t63202, t63204, t63206, t63212, t63214, t63216) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3332(t2, t4560, t580, t1587, t18890, t22, t4595, t52505, t4636, t52219, t15101, t15380);
    (t63193, t63202, t63204, t63206, t63212, t63214, t63216)
}
