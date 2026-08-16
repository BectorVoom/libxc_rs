//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta295(t3995: f64, t9775: f64, t1408: f64, t2681: f64, t820: f64, t1416: f64, t124: f64, t212: f64, t2237: f64, t800: f64, t1376: f64, t123: f64, t125: f64, t2452: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t9776, t9779, t9780, t9784, t9786, t9789) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1285(t3995, t9775, t1408, t2681, t820, t1416, t124, t212, t2237, t800, t1376, t123, t125, t2452, t9720);
    (t9776, t9779, t9780, t9784, t9786, t9789)
}
