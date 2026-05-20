//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta838 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta838<F: Float>(t13760: F, t9765: F, t13756: F, t3989: F, t268: F, t5617: F, t46784: F, t13716: F, t221: F, t3978: F, t3979: F, t124: F, t5658: F, t3938: F, t9816: F, t9818: F, t13847: F, t13848: F, t4057: F, t13962: F, t9962: F, t13845: F, t5675: F, t9840: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48904, t48906, t48908, t48909, t48917, t48919) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966::<F>(t13760, t9765, t13756, t3989, t268, t5617, t46784, t13716, t221, t3978, t3979, t124, t5658);
        let (t48922, t48929, t48937, t48941, t48945) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2967::<F>(t3938, t48919, t9816, t9818, t13847, t13848, t4057, t13962, t9962, t13845, t5675, t9840);
    (t48904, t48906, t48908, t48909, t48917, t48919, t48922, t48929, t48937, t48941, t48945)
}
