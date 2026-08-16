//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1907;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1908;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta420(t33: f64, t1711: f64, t9350: f64, t2: f64, t3841: f64, t1113: f64, t580: f64, t22: f64, t3351: f64, t3842: f64, t516: f64, t5557: f64, t5560: f64, zeta_threshold: f64, t13564: f64, t162: f64, t187: f64, t1857: f64, t3857: f64, t5591: f64, t566: f64, t9375: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13565, t13568, t13569, t13579) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1907(t33, t1711, t9350, t2, t3841, t1113, t580, t22, t3351, t3842, t516, t5557, t5560, zeta_threshold);
        let t13581 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1908(t13564, t13579, t162);
        let (t13583, t13585, t13586, t13593, t13597, t13599, t13600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1909(t13581, t187, t1857, t3857, t5591, t566, t9375, t177, t5566, t762, t1450, t5778);
    (t13565, t13568, t13569, t13581, t13583, t13585, t13586, t13593, t13597, t13599, t13600)
}
