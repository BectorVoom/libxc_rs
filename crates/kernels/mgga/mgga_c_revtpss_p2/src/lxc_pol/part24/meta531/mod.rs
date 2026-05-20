//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta531<F: Float>(t3603: F, t6622: F, t1284: F, t24698: F, t487: F, t83107: F, t22648: F, t602: F, t1469: F, t1486: F, t72: F, t23042: F, t3915: F, t686: F, t22970: F, t9680: F, t22453: F, t49471: F, t1358: F, t212: F, t22964: F, t689: F, t13848: F, t22893: F, t47274: F, t9816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t84645, t84859, t84952, t84967, t85037, t85161, t85475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567::<F>(t3603, t6622, t1284, t24698, t487, t83107, t22648, t602, t1469, t1486, t72, t23042, t3915, t686);
        let (t85480, t85484, t85509, t85514) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1568::<F>(t22970, t686, t72, t9680, t22453, t49471, t1358, t212, t22964, t689, t13848, t22893, t47274, t9816);
    (t84645, t84859, t84952, t84967, t85037, t85161, t85475, t85480, t85484, t85509, t85514)
}
