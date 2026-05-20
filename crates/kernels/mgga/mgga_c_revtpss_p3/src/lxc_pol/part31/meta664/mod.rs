//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2257;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta664<F: Float>(t28264: F, t572: F, t5920: F, t105886: F, t117: F, t2042: F, t22544: F, t26123: F, t5883: F, t7002: F, t101622: F, t1518: F, t28276: F, t4292: F, t109291: F, t109293: F, t109295: F, t109299: F, t109305: F, t109307: F, t1918: F, t2040: F, t22559: F, t22565: F, t28246: F, t5802: F, t6948: F, t7324: F, t7944: F, t1913: F, t7956: F, t101563: F, t105814: F, t109278: F, t109289: F, t1458: F, t1464: F, t1914: F, t1921: F, t2038: F, t2045: F, t22533: F, t22571: F, t28235: F, t28283: F, t3: F, t30161: F, t575: F, t5790: F, t5808: F, t6951: F, t7319: F, t7940: F, t30197: F, t571: F, t6936: F, t7939: F, t2037: F, t101656: F, t101658: F, t101661: F, t101668: F, t101670: F, t101672: F, t101674: F, t1456: F, t6937: F, t7337: F) -> F {
        let (t109310, t109315, t109319, t109322, t109327, t109330) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2254::<F>(t28264, t572, t5920, t105886, t117, t2042, t22544, t26123, t5883, t7002, t101622, t1518);
        let t109334 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2255::<F>(t28276, t4292, t572, t109291, t109293, t109295, t109299, t109305, t109307, t109310, t109315, t109319, t109322, t109327, t109330, t1918, t2040, t22559, t22565, t28246, t5802, t6948, t7324, t7944);
        let t109344 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2256::<F>(t1913, t7956, t101563, t105814, t109278, t109289, t109334, t1458, t1464, t1914, t1921, t2038, t2045, t22533, t22571, t28235, t28283, t3, t30161, t575, t5790, t5808, t6951, t7319, t7940);
        let t109352 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2257::<F>(t30197, t571, t2045, t6936, t1921, t7939, t2037, t6951, t101656, t101658, t101661, t101668, t101670, t101672, t101674, t1456, t6937, t7337);
        let tv4rho3sigma6 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2258::<F>(t109344, t109352);
    tv4rho3sigma6
}
