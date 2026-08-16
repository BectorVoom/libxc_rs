//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 212/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk212(t40: f64, t699: f64, t229: f64, t244: f64, t1: f64, t243: f64, t283: f64, t224: f64, t277: f64, t36: f64, t595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t700 = t40 * t699;
    let t701 = 2.0_f64 * t700;
    let t702 = t229 * t244;
    let t703 = 8.0_f64 * t702;
    let t704 = t243 * t1;
    let t705 = t704 * t283;
    let t706 = 0.36622894612013090108e-3_f64 * t705;
    let t707 = t224 * t277;
    let t708 = 8.0_f64 * t707;
    let t709 = t36 * t595;
    (t700, t701, t702, t703, t704, t705, t706, t708, t709)
}
