//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta585(t102928: f64, t25375: f64, t1957: f64, t28425: f64, t25372: f64, t98809: f64, t25386: f64, t95822: f64, t98815: f64, t95537: f64, t25310: f64, t28360: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t102930, t102934, t102937, t102939, t102941, t102943) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1914(t102928, t25375, t1957, t28425, t25372, t98809, t25386, t95822, t98815, t95537, t25310, t28360);
    (t102930, t102934, t102937, t102939, t102941, t102943)
}
