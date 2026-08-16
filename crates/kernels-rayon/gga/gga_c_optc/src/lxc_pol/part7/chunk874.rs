//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 874/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk874(t2433: f64, t277: f64, t364: f64, t7623: f64, t7626: f64, t7628: f64, t7631: f64, t7666: f64, t7675: f64, t7678: f64, t7684: f64, t7688: f64, t7691: f64, t7694: f64, t7698: f64, t7726: f64, t7833: f64, t8304: f64, t8307: f64, t8381: f64, t8387: f64, t8390: f64, t8393: f64, t95: f64) -> f64 {
    let t8397 = -8.0_f64 / 3.0_f64 * t8304 + 8.0_f64 / 9.0_f64 * t8307 + t8381 * t364 / 2.0_f64 + t7688 - t7694 - t7698 + 200.0_f64 / 81.0_f64 * t2433 * t8387 + 0.51689762869806860992e-2_f64 * t95 * t277 * t8390 * t8393 - t7726 + t7833 + t7623 + t7626 + t7628 + t7631 + t7666 + t7675 - t7678 - t7684 + t7691;
    t8397
}
