//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta653(t1045: f64, t11165: f64, t11156: f64, t3191: f64, t3201: f64, t1021: f64, t11970: f64, t11874: f64, t15688: f64, t11714: f64, t3111: f64, t11722: f64, t3106: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42310, t42316, t42324, t42326, t42328, t42334, t42336) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2440(t1045, t11165, t11156, t3191, t3201, t1021, t11970, t11874, t15688, t11714, t3111, t11722, t3106);
    (t42310, t42316, t42324, t42326, t42328, t42334, t42336)
}
