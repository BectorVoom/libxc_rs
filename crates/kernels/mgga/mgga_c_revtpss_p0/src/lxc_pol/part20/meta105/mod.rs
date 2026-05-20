//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta105 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk598;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk599;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk600;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk601;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta105<F: Float>(t2962: F, t954: F, t944: F, t302: F, t310: F, t2944: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t324: F, t960: F, t964: F, t320: F, t963: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2963, t2966) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk598::<F>(t2962, t954, t944);
        let (t2967, t2968) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk599::<F>(t2966, t302);
        let (t2969, t2970) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk600::<F>(t310);
        let (t2971, t2979) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk601::<F>(t2944, t2970, t2846, t2848, t2855, t2860, t2864);
        let (t2980, t2982, t2985, t2986) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk602::<F>(t2979, t324, t960, t964, t320, t963);
    (t2963, t2966, t2967, t2968, t2969, t2970, t2971, t2979, t2980, t2982, t2985, t2986)
}
