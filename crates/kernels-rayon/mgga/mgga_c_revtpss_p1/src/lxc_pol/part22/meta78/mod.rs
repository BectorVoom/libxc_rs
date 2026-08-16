//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk567;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk568;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk569;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk570;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk571;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta78(t1469: f64, t905: f64, t904: f64, t128: f64, t903: f64, t291: f64, t902: f64, t916: f64, t923: f64, t930: f64, t141: f64, t921: f64, t929: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1592 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk567(t1469, t905);
        let (t1593, t1594, t1596) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk568(t1592, t904, t128, t903);
        let (t1598, t1600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk569(t1596, t291, t1594, t902);
        let (t1601, t1604, t1606, t1607, t1609) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk570(t1600, t916, t923, t1592, t930, t141, t1594, t921, t929);
        let t1610 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk571(t1609, t935);
    (t1592, t1593, t1594, t1596, t1598, t1600, t1601, t1604, t1606, t1607, t1609, t1610)
}
