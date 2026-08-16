//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta449(t5760: f64, t9292: f64, t40921: f64, t5737: f64, t4101: f64, t5740: f64, t9288: f64, t40270: f64, t1892: f64, t9990: f64, t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64, t14159: f64, t3964: f64, t9285: f64, t5600: f64, t1893: f64, t4075: f64, t786: f64, t10115: f64, t1894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49172, t49178, t49203, t49210, t49327, t49354) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412(t5760, t9292, t40921, t5737, t4101, t5740, t9288, t40270, t1892, t9990, t1897, t40317);
        let (t49361, t49432, t49468, t49471, t49474) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413(t10111, t22, t5759, t14159, t3964, t9285, t5600, t9292, t1893, t4075, t786, t10115, t1894);
    (t49172, t49178, t49203, t49210, t49327, t49354, t49361, t49432, t49468, t49471, t49474)
}
