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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta638<F: Float>(t1455: F, t8249: F, t116: F, t29421: F, t10416: F, t13425: F, t13435: F, t13540: F, t13544: F, t1502: F, t18163: F, t2163: F, t2322: F, t2331: F, t27056: F, t27060: F, t27079: F, t29427: F, t29444: F, t29456: F, t4246: F, t4248: F, t4254: F, t4292: F, t4297: F, t651: F, t671: F, t7586: F, t7683: F, t8158: F, t97604: F, t97606: F, t97608: F, t2126: F, t2371: F, t13514: F, t1519: F, t2328: F, t29337: F, t29432: F, t29459: F, t4257: F, t4293: F, t670: F, t8233: F, t97610: F, t97617: F, t97629: F, t97639: F, t97641: F, t97643: F, t97645: F, t97647: F, t97649: F, t97653: F, t97657: F, t97659: F, t2327: F, t8151: F, t10301: F, t29411: F, t2247: F, t29362: F, t38: F, t1923: F, t25102: F, t25110: F, t25114: F, t25117: F, t25150: F, t26782: F, t26789: F, t28089: F, t29372: F, t29375: F, t29412: F, t6954: F, t6960: F, t7575: F, t7709: F, t7719: F, t8144: F, t8147: F, t10309: F, t60224: F, t7565: F, t28150: F, t101156: F, t101337: F, t25120: F, t25159: F, t25162: F, t26749: F, t26755: F, t26792: F, t28133: F, t28147: F, t29364: F, t29367: F, t29380: F, t6963: F, t7566: F, t92588: F, t96827: F, t101214: F, t2122: F, t101172: F, t101176: F, t101182: F, t101187: F, t101399: F, t2123: F, t28105: F, t28109: F, t7706: F, t96792: F, t96810: F, t101129: F, t101132: F, t101190: F, t101193: F, t101350: F, t28112: F, t28116: F, t7576: F, t7579: F, t60221: F, t13272: F, t26754: F, t101139: F, t101323: F, t101357: F, t28141: F, t29388: F, t96773: F, t96776: F, t25163: F, t8143: F, t101226: F, t101200: F, t101230: F, t26783: F, t26786: F, t26795: F, t28119: F, t28154: F, t92565: F, t96760: F, t96765: F, t96824: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104094, t104115, t104135) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2208::<F>(t1455, t8249, t116, t29421, t10416, t13425, t13435, t13540, t13544, t1502, t18163, t2163, t2322, t2331, t27056, t27060, t27079, t29427, t29444, t29456, t4246, t4248, t4254, t4292, t4297, t651, t671, t7586, t7683, t8158, t97604, t97606, t97608);
        let (t104138, t104153) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209::<F>(t2126, t2371, t13514, t1519, t2163, t2322, t2328, t27060, t29337, t29432, t29459, t4257, t4293, t651, t670, t8233, t97610, t97617, t97629, t97639, t97641, t97643, t97645, t97647, t97649, t97653, t97657, t97659);
        let (t104163, t104194) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2210::<F>(t2327, t8151, t10301, t29411, t2247, t29362, t38, t1923, t25102, t25110, t25114, t25117, t25150, t26782, t26789, t28089, t29372, t29375, t29412, t6954, t6960, t7575, t7709, t7719, t8144, t8147);
        let t104222 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211::<F>(t10309, t29411, t60224, t7565, t28150, t7575, t101156, t101337, t25120, t25159, t25162, t26749, t26755, t26792, t28133, t28147, t29364, t29367, t29380, t6963, t7566, t8144, t92588, t96827);
        let t104249 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212::<F>(t101214, t2122, t101172, t101176, t101182, t101187, t101399, t2123, t25162, t26749, t26755, t26792, t28105, t28109, t7566, t7706, t96792, t96810);
        let t104274 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2213::<F>(t101129, t101132, t101190, t101193, t101350, t2123, t25102, t25120, t28112, t28116, t29372, t6963, t7566, t7576, t7579, t8147);
        let t104303 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214::<F>(t60221, t7565, t13272, t26754, t101139, t101323, t101357, t2123, t25110, t25114, t28141, t29375, t29388, t6960, t6963, t7576, t7579, t7706, t96773, t96776);
        let t104330 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215::<F>(t25163, t8143, t101226, t2122, t101200, t101230, t25162, t26783, t26786, t26792, t26795, t28119, t28147, t28154, t29380, t7576, t7579, t7709, t92565, t96760, t96765, t96824);
    (t104094, t104115, t104135, t104138, t104153, t104163, t104194, t104222, t104249, t104274, t104303, t104330)
}
