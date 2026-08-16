//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta306(t11200: f64, t378: f64, t3043: f64, t3042: f64, t993: f64, t1071: f64, t989: f64, t3056: f64, t988: f64, t1031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11239) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1077(t11200, t378, t3043, t3042, t993, t1071, t989, t3056, t988, t1031);
    (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11239)
}
