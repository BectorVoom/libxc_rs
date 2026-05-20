//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta123 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk824;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk825;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk826;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk827;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta123<F: Float>(t2944: F, t954: F, t2846: F, t2904: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F, t944: F, t302: F, t310: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2945, t2950, t2957, t2962) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk824::<F>(t2944, t954, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
        let (t2963, t2966) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk825::<F>(t2962, t954, t944);
        let t2967 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk826::<F>(t2966);
        let t2968 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk827::<F>(t2967, t302);
        let (t2969, t2970) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk828::<F>(t310);
    (t2945, t2950, t2957, t2962, t2963, t2966, t2967, t2968, t2969, t2970)
}
