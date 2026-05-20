//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta729<F: Float>(t49876: F, t22: F, t39454: F, t4398: F, t9387: F, t14362: F, t9575: F, t123: F, t2630: F, t4392: F, t9318: F, t14322: F, t2516: F) -> (F, F, F, F, F, F, F, F) {
        let (t49877, t49886, t49887, t49897, t49926, t49930, t49940, t49957) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2498::<F>(t49876, t22, t39454, t4398, t9387, t14362, t9575, t123, t2630, t4392, t9318, t14322, t2516);
    (t49877, t49886, t49887, t49897, t49926, t49930, t49940, t49957)
}
