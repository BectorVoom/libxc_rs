//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta648<F: Float>(t1149: F, t6471: F, t3384: F, t3435: F, t6470: F, t3433: F, t5104: F, t5108: F, t12230: F, t6438: F, t12227: F, t1187: F, t6519: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20641, t20643, t20644, t20645, t20647, t20648, t20650, t20651, t20652, t20654, t20659) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2592::<F>(t1149, t6471, t3384, t3435, t6470, t3433, t5104, t5108, t12230, t6438, t12227, t1187, t6519);
    (t20641, t20643, t20644, t20645, t20647, t20648, t20650, t20651, t20652, t20654, t20659)
}
