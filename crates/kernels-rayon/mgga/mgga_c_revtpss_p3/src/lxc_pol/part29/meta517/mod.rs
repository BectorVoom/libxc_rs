//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta517(t251: f64, t281: f64, t93238: f64, t1032: f64, t11007: f64, t233: f64, t25372: f64, t1957: f64, t2718: f64, t25386: f64, t786: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t93240, t93281, t93302, t93314, t93317, t93320) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1839(t251, t281, t93238, t1032, t11007, t233, t25372, t1957, t2718, t25386, t786, t860);
    (t93240, t93281, t93302, t93314, t93317, t93320)
}
