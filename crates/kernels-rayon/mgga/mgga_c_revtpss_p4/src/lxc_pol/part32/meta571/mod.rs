//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta571(t101218: f64, t2047: f64, t28154: f64, t95296: f64, t28147: f64, t95319: f64, t28150: f64, t7348: f64, t25162: f64, t116: f64, t28651: f64, t2106: f64, t47672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101938, t101955, t101969, t101970, t101972, t102019, t102070) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895(t101218, t2047, t28154, t95296, t28147, t95319, t28150, t7348, t25162, t116, t28651, t2106, t47672);
    (t101938, t101955, t101969, t101970, t101972, t102019, t102070)
}
