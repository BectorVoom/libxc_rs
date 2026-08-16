//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2325/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2325(t3252: f64, t65: f64, t1100: f64, t1699: f64, t1448: f64, t1907: f64, t4292: f64, t93: f64, t1224: f64, t3698: f64, t1298: f64, t1832: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27531 = t65 * t3252;
    let t27717 = t1699 * t1100;
    let t28198 = t1907 * t1448;
    let t28219 = t93 * t4292;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t29322 = t1832 * t1298;
    (t27531, t27717, t28198, t28219, t29048, t29054, t29322)
}
