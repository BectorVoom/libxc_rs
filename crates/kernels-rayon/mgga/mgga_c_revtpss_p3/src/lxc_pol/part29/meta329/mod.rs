//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1241;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta329(t11043: f64, t786: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t2410: f64, t261: f64, t2832: f64, t892: f64, t2408: f64, t2411: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11044, t11045, t11050, t11051, t11064) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1241(t11043, t786, t2467, t2828, t676, t123, t2465, t2410, t261);
        let (t11075, t11084, t11108, t11121, t11128, t11132) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1242(t2832, t892, t2408, t2411, t3335, t389, t1077, t225, t1071, t3046, t268, t271, t7021);
    (t11044, t11045, t11050, t11051, t11064, t11075, t11084, t11108, t11121, t11128, t11132)
}
