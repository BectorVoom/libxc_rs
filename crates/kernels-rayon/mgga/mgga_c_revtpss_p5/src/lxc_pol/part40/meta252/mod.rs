//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk942;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk943;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk944;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk945;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta252(t1390: f64, t5659: f64, t828: f64, t1883: f64, t221: f64, t4019: f64, t4018: f64, t241: f64, t4000: f64, t820: f64, t550: f64, t72: f64, t245: f64, t125: f64, t1882: f64, t1398: f64, t4003: f64, t1388: f64, t1410: f64, t3931: f64, t3956: f64, t4022: f64, t4064: f64, t5606: f64, t5611: f64, t5614: f64, t5619: f64, t5623: f64, t5625: f64, t5629: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5661, t5665, t5666, t5671, t5672) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk942(t1390, t5659, t828, t1883, t221, t4019, t4018, t241, t4000, t820, t550, t72);
        let t5673 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk943(t245, t5672);
        let t5674 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk944(t125, t1882);
        let t5675 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk945(t1398, t4003);
        let (t5677, t5680) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk946(t5674, t5675, t5673, t1388, t1410, t3931, t3956, t4022, t4064, t5606, t5611, t5614, t5619, t5623, t5625, t5629, t5661, t5666, t5671);
    (t5661, t5665, t5671, t5673, t5674, t5675, t5677, t5680)
}
