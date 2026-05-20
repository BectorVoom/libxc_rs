//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta479<F: Float>(t2439: F, t6467: F, t6464: F, t6461: F, t3383: F, t6433: F, t3432: F, t3520: F, t6513: F, t3495: F, t3476: F, t6481: F) -> (F, F, F, F, F, F, F, F) {
        let (t68583, t68585, t68590, t68792, t68952, t69359, t69371, t69376) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1468::<F>(t2439, t6467, t6464, t6461, t3383, t6433, t3432, t3520, t6513, t3495, t3476, t6481);
    (t68583, t68585, t68590, t68792, t68952, t69359, t69371, t69376)
}
