//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta615(t22271: f64, t27940: f64, t22163: f64, t6871: f64, t94429: f64, t22159: f64, t98115: f64, t22120: f64, t26028: f64, t22076: f64, t22102: f64, t94423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t108512, t108514, t108516, t108518, t108520, t108522, t108524) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1955(t22271, t27940, t22163, t6871, t94429, t22159, t98115, t22120, t26028, t22076, t22102, t94423);
    (t108512, t108514, t108516, t108518, t108520, t108522, t108524)
}
