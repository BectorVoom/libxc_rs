//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2471/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2471(t48041: f64, t4086: f64, t5710: f64, t786: f64, t10014: f64, t14242: f64, t10073: f64, t14225: f64, t1892: f64, t5744: f64, t136: f64, t2457: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48042 = 0.34697458558045176417e-2_f64 * t48041;
    let t48048 = t786 * t4086 * t5710;
    let t48079 = t10014 * t14242;
    let t48080 = 0.39029762157531132076e-1_f64 * t48079;
    let t48081 = t10073 * t14225;
    let t48082 = 0.19514881078765566038e-2_f64 * t48081;
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    let t48089 = t3964 * t5710 * t136 * t2457;
    (t48042, t48048, t48080, t48082, t48083, t48084, t48089)
}
