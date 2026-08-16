//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk563;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta98(t251: f64, t836: f64, t231: f64, t2783: f64, t2782: f64, t233: f64, t860: f64, t869: f64, t689: f64, t136: f64, t2457: f64, t2710: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2784, t2786) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk563(t251, t836, t231, t2783);
        let (t2787, t2789, t2790, t2791, t2793, t2796, t2797, t2798) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk564(t2782, t2786, t233, t860, t869, t689, t136, t251, t2457, t2710, t2783, t786);
    (t2784, t2786, t2787, t2789, t2790, t2791, t2793, t2796, t2797, t2798)
}
