//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta296 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1539;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1540;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta296<F: Float>(t126: F, t3181: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F, t1014: F, t2852: F, t245: F, t3089: F, t3088: F, t3114: F, t11223: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11725, t11732, t11735, t11737, t11765, t11772) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1538::<F>(t126, t3181, t1003, t3080, t221, t346, t68, t345, t1014, t2852, t245, t3089);
        let t11773 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1539::<F>(t11772, t3088);
        let t11774 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1540::<F>(t11773, t3114);
        let t11788 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1541::<F>(t11223, t225);
    (t11725, t11732, t11735, t11737, t11765, t11772, t11773, t11774, t11788)
}
