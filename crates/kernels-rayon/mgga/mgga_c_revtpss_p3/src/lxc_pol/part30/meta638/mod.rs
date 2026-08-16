//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta638(t1455: f64, t8249: f64, t116: f64, t29421: f64, t10416: f64, t13425: f64, t13435: f64, t13540: f64, t13544: f64, t1502: f64, t18163: f64, t2163: f64, t2322: f64, t2331: f64, t27056: f64, t27060: f64, t27079: f64, t29427: f64, t29444: f64, t29456: f64, t4246: f64, t4248: f64, t4254: f64, t4292: f64, t4297: f64, t651: f64, t671: f64, t7586: f64, t7683: f64, t8158: f64, t97604: f64, t97606: f64, t97608: f64, t2126: f64, t2371: f64, t13514: f64, t1519: f64, t2328: f64, t29337: f64, t29432: f64, t29459: f64, t4257: f64, t4293: f64, t670: f64, t8233: f64, t97610: f64, t97617: f64, t97629: f64, t97639: f64, t97641: f64, t97643: f64, t97645: f64, t97647: f64, t97649: f64, t97653: f64, t97657: f64, t97659: f64, t2327: f64, t8151: f64, t10301: f64, t29411: f64, t2247: f64, t29362: f64, t38: f64, t1923: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25150: f64, t26782: f64, t26789: f64, t28089: f64, t29372: f64, t29375: f64, t29412: f64, t6954: f64, t6960: f64, t7575: f64, t7709: f64, t7719: f64, t8144: f64, t8147: f64, t10309: f64, t60224: f64, t7565: f64, t28150: f64, t101156: f64, t101337: f64, t25120: f64, t25159: f64, t25162: f64, t26749: f64, t26755: f64, t26792: f64, t28133: f64, t28147: f64, t29364: f64, t29367: f64, t29380: f64, t6963: f64, t7566: f64, t92588: f64, t96827: f64, t101214: f64, t2122: f64, t101172: f64, t101176: f64, t101182: f64, t101187: f64, t101399: f64, t2123: f64, t28105: f64, t28109: f64, t7706: f64, t96792: f64, t96810: f64, t101129: f64, t101132: f64, t101190: f64, t101193: f64, t101350: f64, t28112: f64, t28116: f64, t7576: f64, t7579: f64, t60221: f64, t13272: f64, t26754: f64, t101139: f64, t101323: f64, t101357: f64, t28141: f64, t29388: f64, t96773: f64, t96776: f64, t25163: f64, t8143: f64, t101226: f64, t101200: f64, t101230: f64, t26783: f64, t26786: f64, t26795: f64, t28119: f64, t28154: f64, t92565: f64, t96760: f64, t96765: f64, t96824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t104094, t104115, t104135) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208(t1455, t8249, t116, t29421, t10416, t13425, t13435, t13540, t13544, t1502, t18163, t2163, t2322, t2331, t27056, t27060, t27079, t29427, t29444, t29456, t4246, t4248, t4254, t4292, t4297, t651, t671, t7586, t7683, t8158, t97604, t97606, t97608);
        let (t104138, t104153) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209(t2126, t2371, t13514, t1519, t2163, t2322, t2328, t27060, t29337, t29432, t29459, t4257, t4293, t651, t670, t8233, t97610, t97617, t97629, t97639, t97641, t97643, t97645, t97647, t97649, t97653, t97657, t97659);
        let (t104163, t104194) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210(t2327, t8151, t10301, t29411, t2247, t29362, t38, t1923, t25102, t25110, t25114, t25117, t25150, t26782, t26789, t28089, t29372, t29375, t29412, t6954, t6960, t7575, t7709, t7719, t8144, t8147);
        let t104222 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211(t10309, t29411, t60224, t7565, t28150, t7575, t101156, t101337, t25120, t25159, t25162, t26749, t26755, t26792, t28133, t28147, t29364, t29367, t29380, t6963, t7566, t8144, t92588, t96827);
        let t104249 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212(t101214, t2122, t101172, t101176, t101182, t101187, t101399, t2123, t25162, t26749, t26755, t26792, t28105, t28109, t7566, t7706, t96792, t96810);
        let t104274 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213(t101129, t101132, t101190, t101193, t101350, t2123, t25102, t25120, t28112, t28116, t29372, t6963, t7566, t7576, t7579, t8147);
        let t104303 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214(t60221, t7565, t13272, t26754, t101139, t101323, t101357, t2123, t25110, t25114, t28141, t29375, t29388, t6960, t6963, t7576, t7579, t7706, t96773, t96776);
        let t104330 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215(t25163, t8143, t101226, t2122, t101200, t101230, t25162, t26783, t26786, t26792, t26795, t28119, t28147, t28154, t29380, t7576, t7579, t7709, t92565, t96760, t96765, t96824);
    (t104094, t104115, t104135, t104138, t104153, t104163, t104194, t104222, t104249, t104274, t104303, t104330)
}
