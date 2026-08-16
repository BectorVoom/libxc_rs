//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk537;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk538;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta93<F: Float>(t2922: F, t275: F, t290: F, t2846: F, t307: F, t944: F, t302: F, t2904: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2923, t2924) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk537::<F>(t2922, t275);
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk538::<F>(t290);
        let (t2930, t2941, t2942, t2943, t2950, t2957, t2966) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539::<F>(t2846, t307, t944, t302, t2904);
    (t2923, t2924, t2925, t2926, t2930, t2941, t2942, t2943, t2950, t2957, t2966)
}
