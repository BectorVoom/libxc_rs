//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta627 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2247;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2248;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2249;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2250;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2251;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2252;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2253;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2254;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2255;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2256;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2257;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta627<F: Float>(t27799: F, t98779: F, t1711: F, t2394: F, t2430: F, t27375: F, t94245: F, t61155: F, t2832: F, t1113: F, t4537: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t25767: F, t27364: F, t27382: F, t27777: F, t27802: F, t27810: F, t27817: F, t4541: F, t51780: F, t7087: F, t7091: F, t7783: F, t7863: F, t99542: F, t33: F, t265: F, t502: F, t100973: F, t101021: F, t101064: F, t100927: F, t13312: F, t1469: F, t2003: F, t2258: F, t25792: F, t27822: F, t4186: F, t57: F, t606: F, t7215: F, t7877: F, dens_threshold: F, rho1: F, zeta_threshold: F, t28182: F, t7235: F, t13392: F, t603: F, t13396: F, t13405: F, t1928: F, t25140: F, t25143: F, t25147: F, t28112: F, t28116: F, t28119: F, t6974: F, t6978: F, t7709: F, t4237: F, t644: F, t77: F, t1497: F, t2311: F, t4241: F, t640: F, t13420: F, t84: F, t25099: F, t25106: F, t28086: F, t28090: F, t28105: F, t28109: F, t6958: F, t6963: F, t7706: F, t92644: F, t92702: F, t10298: F, t1470: F, t2242: F, t4181: F, t4187: F, t28108: F, t2315: F, t7705: F, t28150: F, t6973: F, t6977: F, t1926: F, t1927: F, t25163: F, t7715: F, t10309: F, t25157: F, t25162: F, t28147: F, t28151: F, t32592: F, t92565: F, t92588: F, t7719: F, t13272: F, t607: F, t2248: F, t10301: F, t2247: F, t25150: F, t25164: F, t28154: F, t7702: F, t7716: F, t92570: F, t92573: F, t92577: F, t92585: F, t92690: F, t13388: F, t76: F, t15936: F, t1923: F, t25129: F, t25132: F, t25139: F, t25146: F, t28077: F, t28078: F, t28081: F, t28089: F, t6954: F, t6968: F, t72: F, t7720: F, t92597: F, t92600: F, t92605: F, t92612: F, t60221: F, t6957: F, t13269: F, t25105: F, t28126: F, t1493: F, t25102: F, t25110: F, t25159: F, t28127: F, t28133: F, t6960: F, t92666: F, t92699: F, t60224: F, t2259: F, t4173: F, t38: F, t60248: F, t25114: F, t25120: F, t28093: F, t28138: F, t2251: F, t28076: F, t28104: F, t25117: F, t28141: F, t92684: F, t92687: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t101105 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2247::<F>(t27799, t98779, t1711, t2394, t2430, t27375, t94245, t61155, t2832, t1113, t4537, t1940, t1963, t2403, t25206, t25440, t25767, t27364, t27382, t27777, t27802, t27810, t27817, t4541, t51780, t7087, t7091, t7783, t7863, t99542);
        let t101120 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2248::<F>(t33, t265, t502, t100973, t101021, t101064, t101105, t100927, t13312, t1469, t2003, t2258, t25792, t27822, t4186, t57, t606, t7215, t7877, dens_threshold, rho1, zeta_threshold);
        let (t101124, t101152) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2249::<F>(t28182, t7235, t13392, t603, t13396, t13405, t1928, t25140, t25143, t25147, t28112, t28116, t28119, t6974, t6978, t7709);
        let t101185 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2250::<F>(t4237, t644, t77, t1497, t2311, t4241, t640, t13420, t84, t25099, t25106, t28086, t28090, t28105, t28109, t6958, t6963, t7706, t92644, t92702);
        let (t101187, t101190, t101193, t101200, t101204, t101211) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2251::<F>(t10298, t1470, t2242, t4181, t4187, t28108, t644, t77, t2315, t7705, t28150, t6973);
        let t101225 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2252::<F>(t1497, t6977, t1926, t1927, t4241, t25163, t7715, t101187, t101190, t101193, t101200, t101204, t101211, t10309, t1928, t25157, t25162, t28147, t28151, t32592, t92565, t92588);
        let (t101227, t101230, t101234, t101237, t101240) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2253::<F>(t644, t7719, t1926, t13272, t607, t2248, t77, t7705, t10301, t1470, t2247, t4181);
        let t101259 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2254::<F>(t2247, t4187, t10309, t1470, t101227, t101230, t101234, t101237, t101240, t25147, t25150, t25162, t25164, t28154, t7702, t7716, t92570, t92573, t92577, t92585, t92690);
        let t101309 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2255::<F>(t13388, t76, t13312, t13392, t13396, t1469, t15936, t1923, t1926, t1927, t25129, t25132, t25139, t25146, t25150, t28077, t28078, t28081, t28086, t28089, t28090, t4181, t4186, t6954, t6963, t6968, t6973, t6977, t72, t7715, t7719, t7720, t92597, t92600, t92605, t92612);
        let t101340 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2256::<F>(t60221, t6957, t13269, t607, t13272, t25105, t10309, t28126, t1493, t2248, t77, t1928, t25099, t25102, t25106, t25110, t25157, t25159, t28081, t28127, t28133, t6960, t6963, t7706, t7720, t92666, t92699);
        let t101371 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2257::<F>(t60224, t6957, t1493, t2315, t77, t2259, t4173, t38, t60248, t1928, t25114, t25120, t25140, t25143, t25159, t28093, t28127, t28138, t6958, t6974, t6978, t7702, t7716, t7720);
        let t101402 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2258::<F>(t2251, t4173, t10301, t28126, t2247, t28076, t38, t28104, t644, t77, t1928, t25102, t25110, t25117, t25157, t28138, t28141, t28147, t6960, t6974, t6978, t7716, t7720, t92684, t92687);
    (t101120, t101124, t101152, t101185, t101225, t101259, t101309, t101340, t101371, t101402)
}
