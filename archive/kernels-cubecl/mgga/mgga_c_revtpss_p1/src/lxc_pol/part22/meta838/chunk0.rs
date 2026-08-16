//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2966/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966<F: Float>(t13760: F, t9765: F, t13756: F, t3989: F, t268: F, t5617: F, t46784: F, t13716: F, t221: F, t3978: F, t3979: F, t124: F, t5658: F) -> (F, F, F, F, F, F) {
    let t48904 = t9765 * t13760;
    let t48906 = t3989 * t13756;
    let t48908 = t5617 * t268;
    let t48909 = t46784 * t48908;
    let t48917 = t3978 * t3979 * t221 * t13716;
    let t48919 = t124 * t5658;
    (t48904, t48906, t48908, t48909, t48917, t48919)
}
