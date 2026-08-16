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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2222;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta658(t1497: f64, t4237: f64, t77: f64, t1493: f64, t4241: f64, t5872: f64, t640: f64, t21809: f64, t84: f64, t1925: f64, t2247: f64, t5819: f64, t1469: f64, t603: f64, t4186: f64, t2242: f64, t5826: f64, t19680: f64, t1928: f64, t25099: f64, t25106: f64, t29544: f64, t29548: f64, t6958: f64, t6960: f64, t21663: f64, t607: f64, t13272: f64, t28126: f64, t29524: f64, t38: f64, t5868: f64, t644: f64, t101320: f64, t28127: f64, t28133: f64, t28138: f64, t28141: f64, t29526: f64, t29529: f64, t29533: f64, t6963: f64, t7706: f64, t7716: f64, t7720: f64, t13269: f64, t1470: f64, t4173: f64, t4181: f64, t4187: f64, t21698: f64, t101326: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t29554: f64, t6974: f64, t6978: f64, t101385: f64, t101391: f64, t28078: f64, t28081: f64, t28086: f64, t29538: f64, t7709: f64, t5816: f64, t29561: f64, t7705: f64, t1927: f64, t1926: f64, t101219: f64, t101227: f64, t101237: f64, t101240: f64, t101243: f64, t25157: f64, t28090: f64, t28151: f64, t28154: f64, t29562: f64, t92568: f64, t92684: f64, t92687: f64, t92690: f64, t101211: f64, t101215: f64, t101342: f64, t18281: f64, t1923: f64, t19661: f64, t19666: f64, t25129: f64, t25132: f64, t28077: f64, t28093: f64, t28147: f64, t29525: f64, t5825: f64, t6954: f64, t6968: f64, t6977: f64, t72: f64, t7702: f64, t7719: f64, t92600: f64, t92605: f64, t92612: f64, t21804: f64, t76: f64, t60670: f64, t28089: f64, t29513: f64, t29532: f64, t29551: f64, t6973: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t108733, t108737, t108745, t108749, t108753) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2222(t1497, t4237, t77, t1493, t4241, t5872, t640, t21809, t84, t1925, t2247, t5819);
        let t108768 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2223(t1469, t1925, t603, t4186, t77, t84, t2242, t5826, t19680, t108733, t108737, t108745, t108749, t108753, t1928, t25099, t25106, t29544, t29548, t6958, t6960);
        let t108799 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2224(t21663, t607, t13272, t28126, t2247, t29524, t38, t5868, t644, t77, t101320, t1928, t28127, t28133, t28138, t28141, t29526, t29529, t29533, t6958, t6960, t6963, t7706, t7716, t7720);
        let t108829 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225(t13269, t1470, t4173, t4181, t4187, t21698, t603, t101326, t1928, t28105, t28109, t28112, t28116, t28119, t28138, t29554, t6974, t6978, t7706, t7716);
        let t108854 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2226(t101385, t101391, t28078, t28081, t28086, t28105, t28109, t28112, t28116, t28119, t28127, t29538, t6974, t6978, t7706, t7709, t7720);
        let t108889 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227(t5816, t640, t77, t29561, t644, t4241, t7705, t1927, t1926, t101219, t101227, t101237, t101240, t101243, t25157, t28090, t28151, t28154, t29562, t7709, t92568, t92684, t92687, t92690);
        let t108931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228(t101211, t101215, t101342, t18281, t1923, t1927, t19661, t19666, t19680, t25129, t25132, t28077, t28081, t28086, t28090, t28093, t28147, t28154, t29525, t29526, t29529, t5819, t5825, t6954, t6968, t6977, t72, t7702, t7719, t7720, t92600, t92605, t92612);
        let t108963 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2229(t21804, t76, t2242, t5819, t38, t60670, t1923, t1926, t1928, t28078, t28089, t28093, t29513, t29532, t29533, t29551, t6954, t6973, t6974, t6978, t7702, t7715, t7716);
    (t108768, t108799, t108829, t108854, t108889, t108931, t108963)
}
