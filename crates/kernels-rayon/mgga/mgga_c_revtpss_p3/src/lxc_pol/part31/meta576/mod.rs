//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1993;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta576(t10510: f64, t25399: f64, t10115: f64, t1951: f64, t7058: f64, t92871: f64, t1032: f64, t11007: f64, t233: f64, t25372: f64, t10509: f64, t25377: f64, t25375: f64, t1957: f64, t2718: f64, t25386: f64, t25331: f64, t25365: f64, t786: f64, t860: f64, t25410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93273, t93276, t93278, t93280, t93281, t93285) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1993(t10510, t25399, t10115, t1951, t7058, t92871, t1032, t11007, t233, t25372, t10509, t25377);
        let (t93286, t93302, t93306, t93314, t93317, t93320, t93321) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1994(t25375, t93285, t1957, t2718, t25386, t25331, t25365, t25372, t93280, t786, t860, t25410);
    (t93273, t93276, t93278, t93281, t93285, t93286, t93302, t93306, t93314, t93317, t93320, t93321)
}
