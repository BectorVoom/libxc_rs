//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 872/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk872(t1097: f64, t2647: f64, t2674: f64, t1110: f64, t1101: f64, t2754: f64, t2757: f64, t1068: f64, t2679: f64, t7: f64, t132: f64, t2687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7269 = t2647 * t2674 * t1097;
    let t7271 = 0.35089341735807877242e1_f64 * t1110 * t7269;
    let t7272 = t2754 * t1101;
    let t7274 = t2757 * t1101;
    let t7276 = t2754 * t1068;
    let t7278 = t2757 * t1068;
    let t7281 = 1.0_f64 / t2679 / t7;
    let t7292 = 1.0_f64 / t2687 / t132;
    (t7269, t7271, t7272, t7274, t7276, t7278, t7281, t7292)
}
