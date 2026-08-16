//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1019;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta228(t10741: f64, t2674: f64, t2735: f64, t2783: f64, t2664: f64, t808: f64, t2693: f64, t2710: f64, t2713: f64, t2706: f64, t775: f64, t800: f64, t810: f64, t9784: f64, t9789: f64, t235: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10742, t10744, t10745, t10746, t10749, t10752) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1019(t10741, t2674, t2735, t2783, t2664, t808, t2693, t2710, t2713, t2706, t775, t800);
        let (t10756, t10758, t10759, t10760) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1020(t810, t9784, t9789, t235, t2783, t2453);
    (t10742, t10744, t10745, t10746, t10749, t10752, t10756, t10758, t10759, t10760)
}
