//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta393(t13790: f64, t13791: f64, t13789: f64, t3957: f64, t5690: f64, t1873: f64, t9741: f64, t5651: f64, t808: f64, t9736: f64, t241: f64, t820: f64, t9991: f64, t3923: f64, t9994: f64, t5673: f64, t5674: f64, t5697: f64, t9962: f64, t5701: f64, t13778: f64, t13779: f64, t13781: f64, t13786: f64, t3934: f64, t5671: f64, t9735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13792, t13793, t13797, t13798, t13800, t13801, t13804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1967(t13790, t13791, t13789, t3957, t5690, t1873, t9741, t5651, t808, t9736, t241, t820, t9991);
        let t13805 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1968(t3923, t9994);
        let (t13807, t13810, t13813, t13814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1969(t13805, t5673, t5674, t5697, t9962, t5701, t13778, t13779, t13781, t13786, t13793, t13797, t13798, t13801, t13804, t3934, t5671, t9735);
    (t13792, t13793, t13797, t13798, t13800, t13801, t13804, t13805, t13807, t13810, t13813, t13814)
}
