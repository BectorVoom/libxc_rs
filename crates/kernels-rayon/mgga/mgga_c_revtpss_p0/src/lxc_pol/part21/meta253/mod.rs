//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta253(t30: f64, t1448: f64, t4144: f64, t4146: f64, t565: f64, t1333: f64, t3860: f64, t4147: f64, t513: f64, t3874: f64, t605: f64, t1344: f64, t2257: f64, t9336: f64, t9344: f64, zeta_threshold: f64, t33: f64, t516: f64, t1113: f64, t3881: f64, t1348: f64, t3351: f64, t9351: f64, t9357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9590, t9593, t9597, t9598, t9599, t9603, t9605, t9614) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1442(t30, t1448, t4144, t4146, t565, t1333, t3860, t4147, t513, t3874, t605, t1344, t2257, t9336, t9344, zeta_threshold);
        let (t9615, t9617, t9628) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1443(t33, t516, t1113, t3881, t1348, t3351, t9351, t9357, t9614, zeta_threshold);
    (t9590, t9593, t9597, t9598, t9599, t9603, t9605, t9615, t9617, t9628)
}
