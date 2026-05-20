//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta84 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk501;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk502;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk503;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk504;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta84<F: Float>(t221: F, t2485: F, t837: F, t2484: F, t737: F, t744: F, t185: F) -> (F, F, F, F, F, F, F, F) {
        let (t2487, t2488, t2490) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk501::<F>(t221, t2485, t837, t2484, t737);
        let t2491 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk502::<F>(t2490);
        let t2492 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk503::<F>(t744);
        let (t2494, t2495) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk504::<F>(t185);
        let t2496 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk505::<F>(t2491, t2492, t2495);
    (t2487, t2488, t2490, t2491, t2492, t2494, t2495, t2496)
}
