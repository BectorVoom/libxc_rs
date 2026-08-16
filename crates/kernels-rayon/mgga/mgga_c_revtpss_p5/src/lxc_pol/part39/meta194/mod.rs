//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta194(t4292: f64, t508: f64, t1843: f64, t670: f64, t2616: f64, t2524: f64, t1534: f64, t72: f64, t757: f64, t1469: f64, t750: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4293, t4297, t4300, t4301, t4302, t4304, t4305, t4306) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk815(t4292, t508, t1843, t670, t2616, t2524, t1534, t72, t757, t1469, t750, t706);
    (t4293, t4297, t4300, t4301, t4302, t4304, t4305, t4306)
}
