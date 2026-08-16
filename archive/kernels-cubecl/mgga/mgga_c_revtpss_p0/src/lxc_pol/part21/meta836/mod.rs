//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta836 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta836<F: Float>(t12248: F, t16661: F, t3385: F, t12357: F, t1733: F, t3384: F, t12228: F, t12592: F, t5192: F, t1765: F, t45319: F, t12411: F, t17092: F, t12415: F, t16840: F, t56262: F, t56264: F, t56268: F, t56271: F, t56275: F, t56277: F, t56279: F, t56281: F, t56283: F, t56286: F, t56290: F, t57794: F, t57799: F) -> (F, F, F, F, F, F, F, F) {
        let (t57802, t57805, t57808, t57810, t57812, t57814) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135::<F>(t12248, t16661, t3385, t12357, t1733, t3384, t12228, t12592, t5192, t1765, t45319, t12411, t17092);
        let (t57816, t57817) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3136::<F>(t12415, t16840, t56262, t56264, t56268, t56271, t56275, t56277, t56279, t56281, t56283, t56286, t56290, t57794, t57799, t57802, t57805, t57808, t57810, t57812, t57814);
    (t57802, t57805, t57808, t57810, t57812, t57814, t57816, t57817)
}
