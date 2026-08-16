//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta321<F: Float>(t236: F, t9646: F, t243: F, t9721: F, t268: F, t2479: F, t2652: F, t207: F, t242: F, t240: F, t72: F, t136: F, t2476: F) -> (F, F, F, F, F, F, F, F) {
        let (t10688, t10689, t10692, t10693, t10696, t10697, t10698, t10703) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1765::<F>(t236, t9646, t243, t9721, t268, t2479, t2652, t207, t242, t240, t72, t136, t2476);
    (t10688, t10689, t10692, t10693, t10696, t10697, t10698, t10703)
}
