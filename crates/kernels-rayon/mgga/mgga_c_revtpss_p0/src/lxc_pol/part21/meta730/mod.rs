//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2574;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta730(t221: f64, t4018: f64, t4019: f64, t9891: f64, t1389: f64, t3964: f64, t40604: f64, t3961: f64, t9741: f64, t10111: f64, t22: f64, t4092: f64, t39515: f64, t4083: f64, t10043: f64, t9303: f64, t10014: f64, t10019: f64, t268: f64, t4101: f64, t543: f64, t675: f64, t9890: f64, t10139: f64, t281: f64, t4056: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47333, t47337, t47338, t47348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2574(t221, t4018, t4019, t9891, t1389, t3964, t40604, t3961, t9741, t10111, t22, t4092);
        let (t47351, t47352, t47354, t47359, t47364) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2575(t39515, t4083, t10043, t9303, t10014, t10019, t268, t4101, t543, t675, t9890, t10139, t281, t4056, t68);
    (t47333, t47337, t47338, t47348, t47351, t47352, t47354, t47359, t47364)
}
