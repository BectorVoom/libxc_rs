//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk780;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta180(t3951: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64, t1372: f64, t124: f64, t3889: f64, t800: f64, t2453: f64, t546: f64, t1389: f64, t2713: f64, t1414: f64, t828: f64, t2668: f64, t550: f64, t816: f64, t1379: f64, t1408: f64, t2482: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3952, t3953, t3956, t3957, t3958, t3961, t3964) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk780(t3951, t547, t807, t2700, t535, t1369, t794, t1372, t124, t3889, t800, t2453, t546);
        let (t3967, t3970, t3976, t3978) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk781(t1389, t2713, t3964, t1414, t3889, t828, t2668, t550, t816, t1379, t1408, t2482, t27);
    (t3952, t3953, t3956, t3957, t3958, t3961, t3964, t3967, t3970, t3976, t3978)
}
