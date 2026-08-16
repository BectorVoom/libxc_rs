//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk471;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk472;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta78(t251: f64, t785: f64, t780: f64, t2439: f64, t211: f64, t784: f64, t209: f64, t252: f64, t136: f64, t257: f64, t124: f64, t137: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2440, t2441, t2443, t2452) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk471(t251, t785, t780, t2439, t211, t784);
        let t2453 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk472(t209, t2452);
        let (t2454, t2455, t2456, t2457) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk473(t2453, t252, t136, t257, t124, t137, t68);
    (t2440, t2441, t2443, t2452, t2453, t2454, t2455, t2456, t2457)
}
