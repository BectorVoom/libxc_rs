//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1645;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta334(t11300: f64, t2926: f64, t11299: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64, t923: f64, t11156: f64, t2908: f64, t141: f64, t11165: f64, t930: f64, t2912: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11301, t11303, t11304, t11315) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1645(t11300, t2926, t11299, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let (t11316, t11318, t11319, t11321, t11322, t11326) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1646(t11315, t923, t11156, t2908, t141, t11165, t930, t2912, t698);
    (t11301, t11303, t11304, t11315, t11316, t11318, t11319, t11321, t11322, t11326)
}
