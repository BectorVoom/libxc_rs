//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta895 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta895(t61178: f64, t61180: f64, t39860: f64, t18263: f64, t4305: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t49958: f64, t49964: f64, t49982: f64, t190: f64, t706: f64, t76397: f64, t40092: f64, t40094: f64, t14330: f64, t18305: f64, t4181: f64, t61201: f64, t157: f64, t23121: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76976, t76977, t76978, t76980, t76981) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853(t61178, t61180, t39860, t18263, t4305, t39783, t39786, t39791, t39795, t39799, t39807, t39813, t39818, t39823, t40084, t49958, t49964, t49982);
        let (t76986, t76987, t76988, t76991, t76992, t76995) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854(t190, t706, t76397, t40092, t40094, t14330, t18305, t4181, t61201, t157, t23121, t606);
    (t76976, t76977, t76978, t76980, t76981, t76986, t76987, t76988, t76991, t76992, t76995)
}
