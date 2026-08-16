//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1421;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta243(t1317: f64, t3853: f64, t3829: f64, t4140: f64, t5536: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9365: f64, t9374: f64, t9376: f64, t9389: f64, t9391: f64, t9394: f64, t1320: f64, t4029: f64, t1353: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t9395, t9396, t9397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1421(t1317, t3853, t3829, t4140, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9365, t9374, t9376, t9389, t9391, t9394);
        let (t9398, t9399, t9400) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1422(t1320, t4029, t1353, t3829);
    (t9395, t9396, t9397, t9398, t9399, t9400)
}
