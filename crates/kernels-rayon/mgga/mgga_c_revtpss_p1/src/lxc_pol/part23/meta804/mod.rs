//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta804 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta804(t231: f64, t2782: f64, t2783: f64, t6041: f64, t836: f64, t61756: f64, t2797: f64, t136: f64, t2457: f64, t2710: f64, t10535: f64, t5978: f64, t5977: f64, t860: f64, t18657: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t18750: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62693, t62697, t62716, t62723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634(t231, t2782, t2783, t6041, t836, t61756, t2797, t136, t2457, t2710, t10535, t5978);
        let (t62760, t62763, t62775, t62777, t62788) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2635(t5977, t860, t231, t2782, t2783, t18657, t233, t689, t869, t10069, t18750, t822);
    (t62693, t62697, t62716, t62723, t62760, t62763, t62775, t62777, t62788)
}
