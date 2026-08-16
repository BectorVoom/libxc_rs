//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1181/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1181(t20800: f64, t25: f64, t20947: f64, t25014: f64, t1408: f64, t5527: f64, t5664: f64, t5660: f64, t20778: f64, t105769: f64, t25927: f64, t105754: f64, t23788: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t105797 = t25 * t20800;
    let t105801 = t25014 * t20947;
    let t105810 = t1408 * t5527;
    let t105814 = t1408 * t5664;
    let t105818 = t1408 * t5660;
    let t105822 = t25 * t20778;
    let t106618 = t25927 * t105769;
    let t106621 = t23788 * t105754;
    (t105797, t105801, t105810, t105814, t105818, t105822, t106618, t106621)
}
