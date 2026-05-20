//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk591;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk592;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta103<F: Float>(t2853: F, t2908: F, t141: F, t2858: F, t930: F, t2862: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2892: F, t2898: F, t2900: F, t2905: F, t2906: F, t935: F, t915: F, t913: F, t275: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2909, t2910, t2912, t2913, t2915, t2916, t2918) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk591::<F>(t2853, t2908, t141, t2858, t930, t2862, t2848, t2855, t2860, t2864, t2882, t2890, t2892, t2898, t2900, t2905, t2906);
        let (t2919, t2921, t2922) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk592::<F>(t2918, t935, t915, t913);
        let (t2923, t2924) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk593::<F>(t2922, t275);
    (t2909, t2910, t2912, t2913, t2915, t2916, t2918, t2919, t2921, t2922, t2923, t2924)
}
