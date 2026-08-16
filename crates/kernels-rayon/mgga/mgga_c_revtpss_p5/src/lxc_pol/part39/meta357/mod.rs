//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1226;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1227;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta357(t14609: f64, t14610: f64, t14612: f64, t14630: f64, t225: f64, t73: f64, t830: f64, t1544: f64, t2475: f64, t2394: f64, t4343: f64, t853: f64, t775: f64, t2430: f64, t4416: f64, t14468: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t2634: f64, t2639: f64, t2642: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t833: f64, t231: f64, t10943: f64, t4364: f64, t4365: f64, t124: f64, t1558: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t4423: f64, t837: f64, t2646: f64, t136: f64, t243: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14633, t14643, t14648, t14649, t14652) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1225(t14609, t14610, t14612, t14630, t225, t73, t830, t1544, t2475, t2394, t4343, t853);
        let t14662 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1226(t14652, t775, t2430, t4416, t14468, t832, t14633, t14643, t14649, t1553, t1555, t227, t229, t2634, t2639, t2642, t4409, t4415, t4417, t4420, t830, t833);
        let (t14663, t14668, t14671, t14675, t14676) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1227(t14662, t231, t10943, t4364, t4365, t124, t1558, t10779, t2749, t10777, t125, t4423);
        let (t14678, t14682, t14686, t14690, t14691) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1228(t14676, t4364, t837, t2646, t4365, t136, t243, t220, t14671, t10777, t125, t4343);
    (t14648, t14663, t14668, t14671, t14675, t14676, t14678, t14682, t14686, t14690, t14691)
}
