//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 874/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk874<F: Float>(t2433: F, t277: F, t364: F, t7623: F, t7626: F, t7628: F, t7631: F, t7666: F, t7675: F, t7678: F, t7684: F, t7688: F, t7691: F, t7694: F, t7698: F, t7726: F, t7833: F, t8304: F, t8307: F, t8381: F, t8387: F, t8390: F, t8393: F, t95: F) -> F {
    let t8397 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t8304 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t8307 + t8381 * t364 / F::cast_from(2.0_f64) + t7688 - t7694 - t7698 + F::cast_from(200.0_f64) / F::cast_from(81.0_f64) * t2433 * t8387 + F::cast_from(0.51689762869806860992e-2_f64) * t95 * t277 * t8390 * t8393 - t7726 + t7833 + t7623 + t7626 + t7628 + t7631 + t7666 + t7675 - t7678 - t7684 + t7691;
    t8397
}
