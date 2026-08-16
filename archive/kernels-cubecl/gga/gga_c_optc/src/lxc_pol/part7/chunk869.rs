//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 869/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk869<F: Float>(t7523: F, t7525: F, t7527: F, t7531: F, t7535: F, t7550: F, t7571: F, t7573: F, t7576: F, t7580: F, t7583: F, t8320: F) -> F {
    let t8321 = F::cast_from(0.60319259259259259259e1_f64) * t7523;
    let t8332 = -t8321 - F::cast_from(0.4105e-2_f64) * t7571 + F::cast_from(0.2463e-2_f64) * t7573 + F::cast_from(0.821e-3_f64) * t7576 - F::cast_from(0.54733333333333333333e-3_f64) * t7580 - F::cast_from(0.12315e-2_f64) * t7583 - F::cast_from(0.2585111111111111111e1_f64) * t7525 + F::cast_from(0.19388333333333333333e1_f64) * t7531 + F::cast_from(0.12925555555555555555e1_f64) * t7527 - F::cast_from(0.21542592592592592592e1_f64) * t7535 - F::cast_from(0.19388333333333333333e1_f64) * t7550;
    let t8333 = t8320 + t8332;
    t8333
}
