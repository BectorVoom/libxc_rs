//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2950;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta830(t1882: f64, t4056: f64, t2682: f64, t4000: f64, t5677: f64, t820: f64, t13985: f64, t46740: f64, t1872: f64, t3924: f64, t9816: f64, t9818: f64, t13848: f64, t47274: f64, t9956: f64, t13878: f64, t9765: f64, t13869: f64, t3989: f64, t2661: f64, t5608: f64, t9840: f64, t9934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48475, t48486, t48488, t48494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2950(t1882, t4056, t2682, t4000, t5677, t820, t13985, t46740, t1872, t3924, t9816, t9818);
        let (t48498, t48508, t48510, t48514) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2951(t13848, t47274, t9816, t9956, t13878, t9765, t13869, t3989, t2661, t5608, t9840, t9934);
    (t48475, t48486, t48488, t48494, t48498, t48508, t48510, t48514)
}
