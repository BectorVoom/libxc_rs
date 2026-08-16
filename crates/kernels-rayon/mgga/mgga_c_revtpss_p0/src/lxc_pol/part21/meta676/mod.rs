//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2482;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2483;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2484;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2485;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2486;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta676(t1123: f64, t9292: f64, t2435: f64, t3373: f64, t3369: f64, t12313: f64, t689: f64, t12319: f64, t2439: f64, t3418: f64, t12283: f64, t698: f64, t406: f64, t12555: f64, t3515: f64, t43813: f64, t1126: f64, t12226: f64, t3382: f64, t3431: f64, t408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43888 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2482(t1123, t9292);
        let t43890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2483(t2435, t3373);
        let t43892 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2484(t2435, t3369);
        let t43894 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2485(t12313, t689);
        let t43896 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2486(t12319, t689);
        let (t43911, t43928, t43946, t43977, t43995, t44012, t44017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2487(t2439, t3418, t12283, t698, t406, t12555, t3515, t43813, t1126, t12226, t3382, t3431, t408);
    (t43888, t43890, t43892, t43894, t43896, t43911, t43928, t43946, t43977, t43995, t44012, t44017)
}
