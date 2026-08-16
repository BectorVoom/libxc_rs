//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1541;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1542;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta295(t10518: f64, t2798: f64, t2722: f64, t675: f64, t231: f64, t268: f64, t251: f64, t4503: f64, t786: f64, t2723: f64, t2453: f64, t2797: f64, t281: f64, t68: f64, t836: f64, t2783: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10519, t10521, t10523, t10524, t10529) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1541(t10518, t2798, t2722, t675, t231, t268, t251, t4503);
        let (t10530, t10532, t10533, t10535) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1542(t10529, t786, t10521, t268, t2723, t2453, t2797);
        let (t10538, t10539, t10541, t10542) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1543(t231, t281, t68, t836, t10535, t2783, t860, t786);
    (t10519, t10523, t10524, t10529, t10530, t10532, t10533, t10535, t10538, t10539, t10541, t10542)
}
