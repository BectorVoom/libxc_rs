//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2494;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta727(t49321: f64, t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64, t14188: f64, t2439: f64, t2777: f64, t10073: f64, t14129: f64, t14159: f64, t3964: f64, t9285: f64, t213: f64, t225: f64, t46475: f64, t5600: f64, t9292: f64, t1893: f64, t4075: f64, t786: f64, t10115: f64, t1894: f64, t14094: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49322, t49354, t49361, t49426, t49429, t49432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2494(t49321, t1897, t40317, t10111, t22, t5759, t14188, t2439, t2777, t10073, t14129, t14159, t3964, t9285);
        let (t49439, t49468, t49471, t49474, t49476) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2495(t213, t225, t46475, t5600, t9292, t1893, t4075, t786, t10115, t1894, t14094, t2435);
    (t49322, t49354, t49361, t49426, t49429, t49432, t49439, t49468, t49471, t49474, t49476)
}
