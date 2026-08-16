//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta838 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta838(t13760: f64, t9765: f64, t13756: f64, t3989: f64, t268: f64, t5617: f64, t46784: f64, t13716: f64, t221: f64, t3978: f64, t3979: f64, t124: f64, t5658: f64, t3938: f64, t9816: f64, t9818: f64, t13847: f64, t13848: f64, t4057: f64, t13962: f64, t9962: f64, t13845: f64, t5675: f64, t9840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48904, t48906, t48908, t48909, t48917, t48919) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966(t13760, t9765, t13756, t3989, t268, t5617, t46784, t13716, t221, t3978, t3979, t124, t5658);
        let (t48922, t48929, t48937, t48941, t48945) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2967(t3938, t48919, t9816, t9818, t13847, t13848, t4057, t13962, t9962, t13845, t5675, t9840);
    (t48904, t48906, t48908, t48909, t48917, t48919, t48922, t48929, t48937, t48941, t48945)
}
