//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta148(t1425: f64, t560: f64, t225: f64, t1429: f64, t2435: f64, t1428: f64, t2777: f64, t2439: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4075, t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk673(t1425, t560, t225, t1429, t2435, t1428, t2777, t2439, t1385);
    (t4075, t4076, t4082, t4083, t4085, t4086)
}
