//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta620(t22068: f64, t25972: f64, t25978: f64, t6880: f64, t6856: f64, t1398: f64, t543: f64, t6895: f64, t1907: f64, t5591: f64, t5778: f64, t5920: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t108625, t108627, t108629, t108653, t108682, t108688, t108710) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1961(t22068, t25972, t25978, t6880, t6856, t1398, t543, t6895, t1907, t5591, t5778, t5920, t648);
    (t108625, t108627, t108629, t108653, t108682, t108688, t108710)
}
