//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta548<F: Float>(t61247: F, t61282: F, t50852: F, t50856: F, t61294: F, t61296: F, t39989: F, t40067: F, t40072: F, t40167: F, t40171: F, t62276: F) -> (F, F, F, F, F, F, F, F) {
        let (t87666, t87667, t87668, t87669, t87670, t87671, t87672, t87673) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1621::<F>(t61247, t61282, t50852, t50856, t61294, t61296, t39989, t40067, t40072, t40167, t40171, t62276);
    (t87666, t87667, t87668, t87669, t87670, t87671, t87672, t87673)
}
