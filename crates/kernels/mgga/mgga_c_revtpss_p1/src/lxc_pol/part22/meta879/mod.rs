//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta879 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta879<F: Float>(t14593: F, t2470: F, t874: F, t1558: F, t2482: F, t2801: F, t2815: F, t10547: F, t14606: F, t10538: F, t14605: F, t49180: F, t14586: F, t2645: F, t10529: F, t2782: F, t10535: F, t136: F, t2457: F, t4424: F, t10523: F, t14568: F, t4423: F, t879: F) -> (F, F, F, F, F, F, F, F) {
        let (t51587, t51598, t51600, t51603) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048::<F>(t14593, t2470, t874, t1558, t2482, t2801, t2815, t10547, t14606, t10538, t14605, t49180);
        let (t51610, t51614, t51617, t51621) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3049::<F>(t14586, t2645, t10529, t2782, t10535, t136, t2457, t4424, t10523, t14568, t2482, t2801, t4423, t879);
    (t51587, t51598, t51600, t51603, t51610, t51614, t51617, t51621)
}
