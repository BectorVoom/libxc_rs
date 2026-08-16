//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta804 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta804<F: Float>(t231: F, t2782: F, t2783: F, t6041: F, t836: F, t61756: F, t2797: F, t136: F, t2457: F, t2710: F, t10535: F, t5978: F, t5977: F, t860: F, t18657: F, t233: F, t689: F, t869: F, t10069: F, t18750: F, t822: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t62693, t62697, t62716, t62723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634::<F>(t231, t2782, t2783, t6041, t836, t61756, t2797, t136, t2457, t2710, t10535, t5978);
        let (t62760, t62763, t62775, t62777, t62788) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2635::<F>(t5977, t860, t231, t2782, t2783, t18657, t233, t689, t869, t10069, t18750, t822);
    (t62693, t62697, t62716, t62723, t62760, t62763, t62775, t62777, t62788)
}
