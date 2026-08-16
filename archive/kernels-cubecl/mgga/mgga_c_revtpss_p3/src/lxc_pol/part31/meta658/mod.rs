//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta658 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2222;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta658<F: Float>(t1497: F, t4237: F, t77: F, t1493: F, t4241: F, t5872: F, t640: F, t21809: F, t84: F, t1925: F, t2247: F, t5819: F, t1469: F, t603: F, t4186: F, t2242: F, t5826: F, t19680: F, t1928: F, t25099: F, t25106: F, t29544: F, t29548: F, t6958: F, t6960: F, t21663: F, t607: F, t13272: F, t28126: F, t29524: F, t38: F, t5868: F, t644: F, t101320: F, t28127: F, t28133: F, t28138: F, t28141: F, t29526: F, t29529: F, t29533: F, t6963: F, t7706: F, t7716: F, t7720: F, t13269: F, t1470: F, t4173: F, t4181: F, t4187: F, t21698: F, t101326: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t29554: F, t6974: F, t6978: F, t101385: F, t101391: F, t28078: F, t28081: F, t28086: F, t29538: F, t7709: F, t5816: F, t29561: F, t7705: F, t1927: F, t1926: F, t101219: F, t101227: F, t101237: F, t101240: F, t101243: F, t25157: F, t28090: F, t28151: F, t28154: F, t29562: F, t92568: F, t92684: F, t92687: F, t92690: F, t101211: F, t101215: F, t101342: F, t18281: F, t1923: F, t19661: F, t19666: F, t25129: F, t25132: F, t28077: F, t28093: F, t28147: F, t29525: F, t5825: F, t6954: F, t6968: F, t6977: F, t72: F, t7702: F, t7719: F, t92600: F, t92605: F, t92612: F, t21804: F, t76: F, t60670: F, t28089: F, t29513: F, t29532: F, t29551: F, t6973: F, t7715: F) -> (F, F, F, F, F, F, F) {
        let (t108733, t108737, t108745, t108749, t108753) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2222::<F>(t1497, t4237, t77, t1493, t4241, t5872, t640, t21809, t84, t1925, t2247, t5819);
        let t108768 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223::<F>(t1469, t1925, t603, t4186, t77, t84, t2242, t5826, t19680, t108733, t108737, t108745, t108749, t108753, t1928, t25099, t25106, t29544, t29548, t6958, t6960);
        let t108799 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224::<F>(t21663, t607, t13272, t28126, t2247, t29524, t38, t5868, t644, t77, t101320, t1928, t28127, t28133, t28138, t28141, t29526, t29529, t29533, t6958, t6960, t6963, t7706, t7716, t7720);
        let t108829 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225::<F>(t13269, t1470, t4173, t4181, t4187, t21698, t603, t101326, t1928, t28105, t28109, t28112, t28116, t28119, t28138, t29554, t6974, t6978, t7706, t7716);
        let t108854 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226::<F>(t101385, t101391, t28078, t28081, t28086, t28105, t28109, t28112, t28116, t28119, t28127, t29538, t6974, t6978, t7706, t7709, t7720);
        let t108889 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227::<F>(t5816, t640, t77, t29561, t644, t4241, t7705, t1927, t1926, t101219, t101227, t101237, t101240, t101243, t25157, t28090, t28151, t28154, t29562, t7709, t92568, t92684, t92687, t92690);
        let t108931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228::<F>(t101211, t101215, t101342, t18281, t1923, t1927, t19661, t19666, t19680, t25129, t25132, t28077, t28081, t28086, t28090, t28093, t28147, t28154, t29525, t29526, t29529, t5819, t5825, t6954, t6968, t6977, t72, t7702, t7719, t7720, t92600, t92605, t92612);
        let t108963 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229::<F>(t21804, t76, t2242, t5819, t38, t60670, t1923, t1926, t1928, t28078, t28089, t28093, t29513, t29532, t29533, t29551, t6954, t6973, t6974, t6978, t7702, t7715, t7716);
    (t108768, t108799, t108829, t108854, t108889, t108931, t108963)
}
