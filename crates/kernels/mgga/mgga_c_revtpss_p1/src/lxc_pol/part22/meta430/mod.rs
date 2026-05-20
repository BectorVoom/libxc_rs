//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2052;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2053;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2054;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta430<F: Float>(t14648: F, t2394: F, t4343: F, t853: F, t775: F, t2430: F, t4416: F, t14468: F, t832: F, t14633: F, t14643: F, t1553: F, t1555: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t830: F, t833: F, t231: F, t10943: F, t4364: F, t4365: F, t124: F, t1558: F, t10779: F, t2749: F, t10777: F, t125: F, t4423: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14649, t14652, t14653, t14656, t14659, t14662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2052::<F>(t14648, t2394, t4343, t853, t775, t2430, t4416, t14468, t832, t14633, t14643, t1553, t1555, t227, t229, t2634, t2639, t2642, t4409, t4415, t4417, t4420, t830, t833);
        let t14663 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2053::<F>(t14662, t231);
        let (t14668, t14671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2054::<F>(t10943, t4364, t4365, t124, t1558);
        let (t14673, t14675, t14676) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2055::<F>(t10779, t14671, t2749, t10777, t125, t4423);
    (t14649, t14652, t14653, t14656, t14659, t14662, t14663, t14668, t14671, t14673, t14675, t14676)
}
