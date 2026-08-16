//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1290;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta299(t225: f64, t9801: f64, t4062: f64, t3889: f64, t543: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64, t124: f64, t1398: f64, t3938: f64, t4003: f64, t4056: f64, t2735: f64, t4086: f64, t3994: f64, t808: f64, t521: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9802, t9804, t9810, t9816, t9817, t9818, t9819) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1290(t225, t9801, t4062, t3889, t543, t1386, t2482, t814, t136, t1412, t220, t124, t1398);
        let (t9821, t9822, t9840, t9845, t9847, t9854) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1291(t3938, t9818, t9819, t9816, t4003, t4056, t2735, t4086, t3994, t808, t521, t9342);
    (t9802, t9804, t9810, t9816, t9817, t9818, t9821, t9822, t9840, t9845, t9847, t9854)
}
