//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta289 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1527;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1528;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1529;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1530;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta289<F: Float>(t10355: F, t10356: F, t2275: F, t606: F, t2258: F, t10326: F, t48: F, t58: F, t59: F, t2282: F, t60: F, t10199: F, t10345: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F, t38: F, t2851: F, t78: F, t2299: F, t3361: F, t81: F, t2306: F, t633: F, t637: F, t77: F, t10317: F, t10318: F, t10321: F, t10328: F, t10331: F, t10336: F, t2252: F, t2260: F, t2263: F, t2292: F, t2312: F, t608: F, t628: F, t641: F, t71: F, t85: F, t5: F, t10296: F, t10298: F, t10301: F, t10309: F, t10310: F, t10313: F, t2242: F, t2247: F, t2248: F, t2315: F, t603: F, t644: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10357, t10361, t10364, t10368, t10369, t10373, t10376, t10379) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1527::<F>(t10355, t10356, t2275, t606, t2258, t10326, t48, t58, t59, t2282, t60, t10199);
        let t10380 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1528::<F>(t10345, t10357, t10361, t10364, t10369, t10373, t10376, t10379, t2270, t2276, t2279, t44, t49, t56, t614, t617);
        let (t10381, t10389, t10398, t10406) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1529::<F>(t10380, t38, t2851, t78, t2299, t606, t3361, t81, t2306, t10326, t10356, t2258, t633, t637);
        let (t10407, t10410) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1530::<F>(t10406, t77, t10317, t10318, t10321, t10328, t10331, t10336, t10381, t2252, t2260, t2263, t2292, t2312, t608, t628, t641, t71, t85);
        let t10414 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1531::<F>(t5, t10296, t10298, t10301, t10309, t10310, t10313, t10410, t2242, t2247, t2248, t2315, t603, t644, t91);
    (t10368, t10369, t10373, t10376, t10379, t10380, t10381, t10389, t10398, t10407, t10410, t10414)
}
