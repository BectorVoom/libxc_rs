//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk585;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk586;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta101<F: Float>(t2847: F, t2848: F, t2855: F, t2860: F, t2864: F, t291: F, t910: F, t914: F, t936: F, t287: F, t913: F, t275: F, t934: F, t935: F, t273: F, t276: F, t918: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2866, t2868, t2869, t2871, t2872, t2873, t2874) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk585::<F>(t2847, t2848, t2855, t2860, t2864, t291, t910, t914, t936, t287, t913, t275);
        let t2875 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk586::<F>(t934);
        let (t2876, t2878, t2880, t2881) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk587::<F>(t2875, t935, t2874, t273, t276, t918);
    (t2866, t2868, t2869, t2871, t2872, t2873, t2874, t2875, t2876, t2878, t2880, t2881)
}
