//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1835;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta348<F: Float>(t1063: F, t11727: F, t1007: F, t3083: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F, t247: F, t2858: F, t3109: F, t140: F, t3247: F, t1011: F, t3254: F, t3237: F, t245: F, t3089: F, t3088: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11728, t11730, t11732, t11735, t11737, t11744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1835::<F>(t1063, t11727, t1007, t3083, t1003, t3080, t221, t346, t68, t345, t247, t2858, t3109);
        let (t11745, t11753, t11756, t11763, t11772, t11773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1836::<F>(t1063, t11744, t140, t3247, t1011, t3254, t3237, t245, t3089, t3088);
    (t11728, t11730, t11732, t11735, t11737, t11744, t11745, t11753, t11756, t11763, t11772, t11773)
}
