//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk720;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta138<F: Float>(t5618: F, t807: F, t1868: F, t221: F, t3979: F, t3978: F, t1885: F, t3930: F, t1856: F, t72: F, t757: F, t539: F, t73: F, t1412: F, t1883: F, t4019: F, t4018: F, t241: F, t4000: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5619, t5622, t5623, t5625, t5635, t5636, t5650) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk720::<F>(t5618, t807, t1868, t221, t3979, t3978, t1885, t3930, t1856, t72, t757, t539, t73);
        let (t5651, t5665, t5666, t5671) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk721::<F>(t1412, t1868, t1883, t221, t4019, t4018, t241, t4000, t820);
    (t5619, t5622, t5623, t5625, t5635, t5636, t5650, t5651, t5665, t5666, t5671)
}
