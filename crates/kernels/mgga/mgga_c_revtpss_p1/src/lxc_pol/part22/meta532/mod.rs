//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2327;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2328;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta532<F: Float>(t1263: F, t1794: F, t372: F, t12712: F, t3629: F, t17301: F, t17304: F, t17308: F, t17311: F, t17333: F, t17337: F, t17339: F, t17340: F, t17342: F, t17344: F, t17347: F, t17351: F, t3674: F, t484: F, t11262: F, t1796: F, t1247: F, t1264: F, t16746: F, t247: F, t12915: F, t5230: F, t5384: F, t1770: F, t3140: F, t3609: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17352, t17353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2327::<F>(t1263, t1794, t372);
        let (t17354, t17355, t17358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2328::<F>(t12712, t3629, t17353, t17301, t17304, t17308, t17311, t17333, t17337, t17339, t17340, t17342, t17344, t17347, t17351, t3674, t484);
        let (t17361, t17362, t17369, t17373, t17375, t17376, t17377) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2329::<F>(t11262, t1796, t1247, t1264, t16746, t247, t12915, t5230, t5384, t1770, t3140, t3609);
    (t17352, t17353, t17354, t17355, t17358, t17361, t17362, t17369, t17373, t17375, t17376, t17377)
}
