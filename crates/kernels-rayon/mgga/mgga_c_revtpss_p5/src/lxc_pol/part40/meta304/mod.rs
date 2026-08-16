//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1070;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1071;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta304(t11043: f64, t786: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t2410: f64, t261: f64, t2832: f64, t892: f64, t2408: f64, t2411: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64, t2435: f64, t907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11044, t11045, t11051, t11064, t11075, t11084) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1070(t11043, t786, t2467, t2828, t676, t123, t2465, t2410, t261, t2832, t892, t2408, t2411);
        let (t11108, t11121, t11128, t11132) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1071(t3335, t389, t1077, t225, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1072(t11132, t2435, t907);
    (t11044, t11045, t11051, t11064, t11075, t11084, t11108, t11121, t11128, t11132, t11133, t11134)
}
