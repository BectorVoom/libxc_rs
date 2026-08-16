//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2412;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta638(t123: f64, t2465: f64, t886: f64, t9291: f64, t10982: f64, t860: f64, t9646: f64, t2434: f64, t2828: f64, t10115: f64, t251: f64, t887: f64, t2439: f64, t2440: f64, t2829: f64, t10977: f64, t686: f64, t72: f64, t11061: f64, t11064: f64, t2410: f64, t2832: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41102, t41105, t41115, t41117, t41118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2412(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
        let (t41125, t41129, t41137, t41154, t41161) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413(t2439, t2440, t2829, t10977, t2465, t686, t72, t11061, t11064, t2410, t2832, t775);
    (t41102, t41105, t41115, t41117, t41118, t41125, t41129, t41137, t41154, t41161)
}
