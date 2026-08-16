//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 984/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk984(t238: f64, t242: f64, t9117: f64, t3470: f64, t940: f64, t343: f64, t9027: f64, t9011: f64, t6969: f64, t6972: f64, t7016: f64, t9008: f64, t9029: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9119 = t238 * t242 * t9117;
    let t9121 = t940 * t3470;
    let t9123 = t238 * t242 * t9121;
    let t9125 = t343 * t9027;
    let t9127 = t238 * t242 * t9125;
    let t9134 = 2.0_f64 / 3.0_f64 * t9011;
    let t9135 = -t7016 + 8.0_f64 / 9.0_f64 * t6969 - t6972 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t9008 - t9134 + t9029;
    (t9119, t9121, t9123, t9125, t9127, t9134, t9135)
}
