//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1745;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1746;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1747;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta393<F: Float>(t1263: F, t1794: F, t372: F, t11262: F, t1796: F, t1247: F, t12915: F, t247: F, t5230: F, t5384: F, t1770: F, t3140: F, t3609: F, t12772: F, t5406: F, t3625: F, t1802: F, t474: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17352, t17353) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1745::<F>(t1263, t1794, t372);
        let (t17361, t17362, t17373, t17375, t17376, t17377, t17384) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1746::<F>(t11262, t1796, t1247, t12915, t247, t5230, t5384, t1770, t3140, t3609, t12772, t5406);
        let (t17386, t17394) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1747::<F>(t17384, t3625, t1802, t474);
        let t17395 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1748::<F>(t17394, t3089);
    (t17352, t17353, t17361, t17362, t17373, t17375, t17376, t17377, t17384, t17386, t17394, t17395)
}
