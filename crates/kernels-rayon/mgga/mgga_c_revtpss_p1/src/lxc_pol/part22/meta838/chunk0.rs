//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2966/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2966(t13760: f64, t9765: f64, t13756: f64, t3989: f64, t268: f64, t5617: f64, t46784: f64, t13716: f64, t221: f64, t3978: f64, t3979: f64, t124: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48904 = t9765 * t13760;
    let t48906 = t3989 * t13756;
    let t48908 = t5617 * t268;
    let t48909 = t46784 * t48908;
    let t48917 = t3978 * t3979 * t221 * t13716;
    let t48919 = t124 * t5658;
    (t48904, t48906, t48908, t48909, t48917, t48919)
}
