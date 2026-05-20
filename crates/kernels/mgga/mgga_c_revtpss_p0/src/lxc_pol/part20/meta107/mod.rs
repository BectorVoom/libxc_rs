//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk611;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta107<F: Float>(t300: F, t3018: F, t2980: F, t960: F, t983: F, t2986: F, t2988: F, t973: F, t981: F, t3006: F, t964: F, t3011: F, t3014: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3019, t3021, t3022) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk611::<F>(t300, t3018, t2980, t960);
        let (t3024, t3026, t3028, t3030, t3032, t3033, t3034, t3036, t3042) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk612::<F>(t3022, t983, t2986, t2988, t973, t981, t3006, t964, t3011, t3014, t2846, t2848, t2855, t2860, t2864);
    (t3019, t3021, t3022, t3024, t3026, t3028, t3030, t3032, t3033, t3034, t3036, t3042)
}
