//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2339;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta637<F: Float>(t2452: F, t588: F, t258: F, t2454: F, t2455: F, t39494: F, t10985: F, t11018: F, t10541: F, t2453: F, t231: F, t268: F, t2798: F, t793: F, t836: F, t14545: F, t251: F, t786: F, t4503: F, t860: F, t10115: F, t883: F, t2710: F, t2793: F, t2804: F, t874: F, t9288: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39552, t39554, t39557, t39558, t39575, t39581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2339::<F>(t2452, t588, t258, t2454, t2455, t39494, t10985, t11018, t10541, t2453, t231, t268, t2798, t793, t836);
        let (t39597, t39598, t39609, t39624, t39633, t39635) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2340::<F>(t14545, t251, t786, t4503, t860, t10115, t883, t2710, t2793, t39494, t2804, t874, t9288);
    (t39552, t39554, t39557, t39558, t39575, t39581, t39597, t39598, t39609, t39624, t39633, t39635)
}
