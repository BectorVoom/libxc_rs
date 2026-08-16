//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta336(t11044: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t2410: f64, t261: f64, t2832: f64, t892: f64, t2408: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11045, t11049, t11050, t11051, t11064, t11075, t11084) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1794(t11044, t2467, t2828, t676, t123, t2465, t2410, t261, t2832, t892, t2408, t2411);
    (t11045, t11049, t11050, t11051, t11064, t11075, t11084)
}
