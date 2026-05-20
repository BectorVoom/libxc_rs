//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2812;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta744<F: Float>(t11007: F, t252: F, t786: F, t11006: F, t256: F, t225: F, t2441: F, t39515: F, t10504: F, t138: F, t886: F, t9302: F, t123: F, t2465: F, t9291: F, t10982: F, t860: F, t9646: F, t2434: F, t2828: F, t10115: F, t251: F, t887: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41070, t41078, t41095, t41098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2812::<F>(t11007, t252, t786, t11006, t256, t225, t2441, t39515, t10504, t138, t886, t9302);
        let (t41102, t41105, t41115, t41117, t41118) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2813::<F>(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
    (t41070, t41078, t41095, t41098, t41102, t41105, t41115, t41117, t41118)
}
