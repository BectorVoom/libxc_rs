//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2017;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta592(t1399: f64, t2434: f64, t25880: f64, t25899: f64, t2022: f64, t9646: f64, t9648: f64, t25875: f64, t94394: f64, t46361: f64, t545: f64, t9685: f64, t25895: f64, t1032: f64, t9656: f64, t25894: f64, t25950: f64, t25953: f64, t26069: f64, t94407: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94634, t94635, t94648, t94649, t94656, t94661) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2017(t1399, t2434, t25880, t25899, t2022, t9646, t9648, t25875, t94394, t46361, t545, t9685);
        let (t94662, t94669, t94674, t94677, t94682, t94696) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2018(t25895, t94661, t1032, t9656, t545, t25875, t25894, t25950, t25953, t26069, t94407, t7282, t9646);
    (t94634, t94635, t94648, t94649, t94656, t94661, t94662, t94669, t94674, t94677, t94682, t94696)
}
