//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta471(t2061: f64, t785: f64, t780: f64, t2439: f64, t2435: f64, t7385: f64, t212: f64, t7398: f64, t689: f64, t25219: f64, t25231: f64, t25242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26434, t26435, t26437, t26439, t26446, t26447, t26448, t26450, t26454, t26457) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1700(t2061, t785, t780, t2439, t2435, t7385, t212, t7398, t689, t25219, t25231, t25242);
    (t26434, t26435, t26437, t26439, t26446, t26447, t26448, t26450, t26454, t26457)
}
