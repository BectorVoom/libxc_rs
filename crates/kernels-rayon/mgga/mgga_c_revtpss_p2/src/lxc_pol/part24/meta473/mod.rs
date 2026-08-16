//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1455;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta473(t2439: f64, t2440: f64, t6072: f64, t15003: f64, t51258: f64, t6042: f64, t786: f64, t867: f64, t14485: f64, t14987: f64, t2435: f64, t6093: f64, t6097: f64, t6101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t63050, t63058, t63084, t63099, t63453) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454(t2439, t2440, t6072, t15003, t51258, t6042, t786, t867, t14485, t14987, t2435, t6093);
        let t63459 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1455(t2435, t6097);
        let t63464 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1456(t2435, t6101);
    (t63050, t63058, t63084, t63099, t63453, t63459, t63464)
}
