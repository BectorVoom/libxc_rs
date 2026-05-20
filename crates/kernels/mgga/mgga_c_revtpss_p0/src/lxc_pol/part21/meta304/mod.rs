//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1559;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta304<F: Float>(t10680: F, t236: F, t807: F, t243: F, t247: F, t9949: F, t237: F, t9646: F, t9721: F, t268: F, t2479: F, t2652: F, t207: F, t242: F, t240: F, t72: F, t10627: F, t828: F, t136: F, t2476: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10681, t10682, t10687, t10688, t10689, t10692, t10693) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1559::<F>(t10680, t236, t807, t243, t247, t9949, t237, t9646, t9721, t268, t2479, t2652);
        let (t10696, t10697, t10698, t10700, t10703) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1560::<F>(t207, t242, t240, t72, t10627, t828, t136, t2476);
    (t10681, t10682, t10687, t10688, t10689, t10692, t10693, t10696, t10697, t10698, t10700, t10703)
}
