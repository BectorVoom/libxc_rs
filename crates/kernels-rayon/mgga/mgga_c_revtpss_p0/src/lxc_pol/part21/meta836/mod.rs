//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta836 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta836(t12248: f64, t16661: f64, t3385: f64, t12357: f64, t1733: f64, t3384: f64, t12228: f64, t12592: f64, t5192: f64, t1765: f64, t45319: f64, t12411: f64, t17092: f64, t12415: f64, t16840: f64, t56262: f64, t56264: f64, t56268: f64, t56271: f64, t56275: f64, t56277: f64, t56279: f64, t56281: f64, t56283: f64, t56286: f64, t56290: f64, t57794: f64, t57799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57802, t57805, t57808, t57810, t57812, t57814) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135(t12248, t16661, t3385, t12357, t1733, t3384, t12228, t12592, t5192, t1765, t45319, t12411, t17092);
        let (t57816, t57817) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3136(t12415, t16840, t56262, t56264, t56268, t56271, t56275, t56277, t56279, t56281, t56283, t56286, t56290, t57794, t57799, t57802, t57805, t57808, t57810, t57812, t57814);
    (t57802, t57805, t57808, t57810, t57812, t57814, t57816, t57817)
}
