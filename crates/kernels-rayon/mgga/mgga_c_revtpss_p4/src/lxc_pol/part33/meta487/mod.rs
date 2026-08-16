//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1778;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta487(t676: f64, t837: f64, t25377: f64, t25411: f64, t2718: f64, t867: f64, t1950: f64, t2453: f64, t2458: f64, t25372: f64, t25410: f64, t2411: f64, t7086: f64, t11064: f64, t1962: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25412, t25413, t25414, t25416, t25422, t25424, t25431) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1776(t676, t837, t25377, t25411, t2718, t867, t1950, t2453, t2458, t25372, t25410);
        let (t25432, t25440) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1777(t25413, t25431, t2411, t7086);
        let t25445 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1778(t11064, t1962);
        let t25759 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1779(t2411, t33);
    (t25412, t25413, t25414, t25416, t25422, t25424, t25431, t25432, t25440, t25445, t25759)
}
