//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1044;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta291(t760: f64, t9318: f64, t2251: f64, t750: f64, t2611: f64, t2398: f64, t2615: f64, t2609: f64, t717: f64, t162: f64, t9544: f64, t158: f64, t755: f64, t9586: f64, t2619: f64, t2622: f64, t2390: f64, t72: f64, t757: f64, t2629: f64, t9863: f64, t123: f64, t752: f64, t2630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10554, t10556, t10561, t10563, t10566) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1044(t760, t9318, t2251, t750, t2611, t2398, t2615, t2609, t717, t162, t9544, t158);
        let (t10568, t10569, t10574, t10577, t10579) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1045(t755, t9586, t2619, t2622, t2390, t72, t757, t2629, t9863, t123, t752, t2630);
    (t10554, t10556, t10561, t10563, t10566, t10568, t10569, t10574, t10577, t10579)
}
