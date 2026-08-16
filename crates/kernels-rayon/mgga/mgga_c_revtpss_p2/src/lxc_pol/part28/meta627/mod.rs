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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta627(t27799: f64, t98779: f64, t1711: f64, t2394: f64, t2430: f64, t27375: f64, t94245: f64, t61155: f64, t2832: f64, t1113: f64, t4537: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t25767: f64, t27364: f64, t27382: f64, t27777: f64, t27802: f64, t27810: f64, t27817: f64, t4541: f64, t51780: f64, t7087: f64, t7091: f64, t7783: f64, t7863: f64, t99542: f64, t33: f64, t265: f64, t502: f64, t100973: f64, t101021: f64, t101064: f64, t100927: f64, t13312: f64, t1469: f64, t2003: f64, t2258: f64, t25792: f64, t27822: f64, t4186: f64, t57: f64, t606: f64, t7215: f64, t7877: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t28182: f64, t7235: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t1928: f64, t25140: f64, t25143: f64, t25147: f64, t28112: f64, t28116: f64, t28119: f64, t6974: f64, t6978: f64, t7709: f64, t4237: f64, t644: f64, t77: f64, t1497: f64, t2311: f64, t4241: f64, t640: f64, t13420: f64, t84: f64, t25099: f64, t25106: f64, t28086: f64, t28090: f64, t28105: f64, t28109: f64, t6958: f64, t6963: f64, t7706: f64, t92644: f64, t92702: f64, t10298: f64, t1470: f64, t2242: f64, t4181: f64, t4187: f64, t28108: f64, t2315: f64, t7705: f64, t28150: f64, t6973: f64, t6977: f64, t1926: f64, t1927: f64, t25163: f64, t7715: f64, t10309: f64, t25157: f64, t25162: f64, t28147: f64, t28151: f64, t32592: f64, t92565: f64, t92588: f64, t7719: f64, t13272: f64, t607: f64, t2248: f64, t10301: f64, t2247: f64, t25150: f64, t25164: f64, t28154: f64, t7702: f64, t7716: f64, t92570: f64, t92573: f64, t92577: f64, t92585: f64, t92690: f64, t13388: f64, t76: f64, t15936: f64, t1923: f64, t25129: f64, t25132: f64, t25139: f64, t25146: f64, t28077: f64, t28078: f64, t28081: f64, t28089: f64, t6954: f64, t6968: f64, t72: f64, t7720: f64, t92597: f64, t92600: f64, t92605: f64, t92612: f64, t60221: f64, t6957: f64, t13269: f64, t25105: f64, t28126: f64, t1493: f64, t25102: f64, t25110: f64, t25159: f64, t28127: f64, t28133: f64, t6960: f64, t92666: f64, t92699: f64, t60224: f64, t2259: f64, t4173: f64, t38: f64, t60248: f64, t25114: f64, t25120: f64, t28093: f64, t28138: f64, t2251: f64, t28076: f64, t28104: f64, t25117: f64, t28141: f64, t92684: f64, t92687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t101105 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2247(t27799, t98779, t1711, t2394, t2430, t27375, t94245, t61155, t2832, t1113, t4537, t1940, t1963, t2403, t25206, t25440, t25767, t27364, t27382, t27777, t27802, t27810, t27817, t4541, t51780, t7087, t7091, t7783, t7863, t99542);
        let t101120 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2248(t33, t265, t502, t100973, t101021, t101064, t101105, t100927, t13312, t1469, t2003, t2258, t25792, t27822, t4186, t57, t606, t7215, t7877, dens_threshold, rho1, zeta_threshold);
        let (t101124, t101152) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2249(t28182, t7235, t13392, t603, t13396, t13405, t1928, t25140, t25143, t25147, t28112, t28116, t28119, t6974, t6978, t7709);
        let t101185 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2250(t4237, t644, t77, t1497, t2311, t4241, t640, t13420, t84, t25099, t25106, t28086, t28090, t28105, t28109, t6958, t6963, t7706, t92644, t92702);
        let (t101187, t101190, t101193, t101200, t101204, t101211) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2251(t10298, t1470, t2242, t4181, t4187, t28108, t644, t77, t2315, t7705, t28150, t6973);
        let t101225 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2252(t1497, t6977, t1926, t1927, t4241, t25163, t7715, t101187, t101190, t101193, t101200, t101204, t101211, t10309, t1928, t25157, t25162, t28147, t28151, t32592, t92565, t92588);
        let (t101227, t101230, t101234, t101237, t101240) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2253(t644, t7719, t1926, t13272, t607, t2248, t77, t7705, t10301, t1470, t2247, t4181);
        let t101259 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2254(t2247, t4187, t10309, t1470, t101227, t101230, t101234, t101237, t101240, t25147, t25150, t25162, t25164, t28154, t7702, t7716, t92570, t92573, t92577, t92585, t92690);
        let t101309 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2255(t13388, t76, t13312, t13392, t13396, t1469, t15936, t1923, t1926, t1927, t25129, t25132, t25139, t25146, t25150, t28077, t28078, t28081, t28086, t28089, t28090, t4181, t4186, t6954, t6963, t6968, t6973, t6977, t72, t7715, t7719, t7720, t92597, t92600, t92605, t92612);
        let t101340 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2256(t60221, t6957, t13269, t607, t13272, t25105, t10309, t28126, t1493, t2248, t77, t1928, t25099, t25102, t25106, t25110, t25157, t25159, t28081, t28127, t28133, t6960, t6963, t7706, t7720, t92666, t92699);
        let t101371 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2257(t60224, t6957, t1493, t2315, t77, t2259, t4173, t38, t60248, t1928, t25114, t25120, t25140, t25143, t25159, t28093, t28127, t28138, t6958, t6974, t6978, t7702, t7716, t7720);
        let t101402 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2258(t2251, t4173, t10301, t28126, t2247, t28076, t38, t28104, t644, t77, t1928, t25102, t25110, t25117, t25157, t28138, t28141, t28147, t6960, t6974, t6978, t7716, t7720, t92684, t92687);
    (t101120, t101124, t101152, t101185, t101225, t101259, t101309, t101340, t101371, t101402)
}
