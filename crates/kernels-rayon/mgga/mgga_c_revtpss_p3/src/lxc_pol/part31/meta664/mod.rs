//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2257;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta664(t28264: f64, t572: f64, t5920: f64, t105886: f64, t117: f64, t2042: f64, t22544: f64, t26123: f64, t5883: f64, t7002: f64, t101622: f64, t1518: f64, t28276: f64, t4292: f64, t109291: f64, t109293: f64, t109295: f64, t109299: f64, t109305: f64, t109307: f64, t1918: f64, t2040: f64, t22559: f64, t22565: f64, t28246: f64, t5802: f64, t6948: f64, t7324: f64, t7944: f64, t1913: f64, t7956: f64, t101563: f64, t105814: f64, t109278: f64, t109289: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t2038: f64, t2045: f64, t22533: f64, t22571: f64, t28235: f64, t28283: f64, t3: f64, t30161: f64, t575: f64, t5790: f64, t5808: f64, t6951: f64, t7319: f64, t7940: f64, t30197: f64, t571: f64, t6936: f64, t7939: f64, t2037: f64, t101656: f64, t101658: f64, t101661: f64, t101668: f64, t101670: f64, t101672: f64, t101674: f64, t1456: f64, t6937: f64, t7337: f64) -> f64 {
        let (t109310, t109315, t109319, t109322, t109327, t109330) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254(t28264, t572, t5920, t105886, t117, t2042, t22544, t26123, t5883, t7002, t101622, t1518);
        let t109334 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255(t28276, t4292, t572, t109291, t109293, t109295, t109299, t109305, t109307, t109310, t109315, t109319, t109322, t109327, t109330, t1918, t2040, t22559, t22565, t28246, t5802, t6948, t7324, t7944);
        let t109344 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256(t1913, t7956, t101563, t105814, t109278, t109289, t109334, t1458, t1464, t1914, t1921, t2038, t2045, t22533, t22571, t28235, t28283, t3, t30161, t575, t5790, t5808, t6951, t7319, t7940);
        let t109352 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2257(t30197, t571, t2045, t6936, t1921, t7939, t2037, t6951, t101656, t101658, t101661, t101668, t101670, t101672, t101674, t1456, t6937, t7337);
        let tv4rho3sigma6 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2258(t109344, t109352);
    tv4rho3sigma6
}
