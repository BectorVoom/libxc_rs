//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta390<F: Float>(t2434: F, t2496: F, t2629: F, t676: F, t9419: F, t762: F, t9291: F, t2: F, t588: F, t2576: F, t2565: F, t701: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39427, t39429, t39430, t39432, t39440, t39442, t39454, t39480, t39483) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1299::<F>(t2434, t2496, t2629, t676, t9419, t762, t9291, t2, t588, t2576, t2565, t701);
    (t39427, t39429, t39430, t39432, t39440, t39442, t39454, t39480, t39483)
}
