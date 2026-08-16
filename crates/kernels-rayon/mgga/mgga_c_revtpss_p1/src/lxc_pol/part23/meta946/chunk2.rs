//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3118/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3118(t20619: f64, t57944: f64, t81612: f64, t81614: f64, t81618: f64, t81621: f64, t81623: f64, t81625: f64, t81627: f64, t81629: f64, t81631: f64, t81633: f64, t81635: f64, t81638: f64, t81641: f64, t81646: f64) -> f64 {
    let t82049 = -0.57895126195293126241e3_f64 * t57944 * t20619 + t81612 - t81614 - t81618 - t81621 + t81623 - t81625 + t81627 - t81629 + t81631 - t81633 - t81635 - t81638 + t81641 + t81646;
    t82049
}
