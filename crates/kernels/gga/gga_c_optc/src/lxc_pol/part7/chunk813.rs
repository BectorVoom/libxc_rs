//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 813/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk813<F: Float>(t7592: F, t7523: F, t7525: F, t7527: F, t7531: F, t7535: F, t7541: F, t7547: F, t7550: F, t7576: F, t7580: F, t7583: F, t7596: F, t7599: F) -> F {
    let t7656 = F::new(0.36514074074074074075e0) * t7592;
    let t7657 = F::new(0.93011851851851851854e0) * t7523;
    let t7662 = F::new(0.5477111111111111111e-1) * t7576 - F::new(0.36514074074074074075e-1) * t7580 - F::new(0.82156666666666666667e-1) * t7583 - F::new(0.39862222222222222223e0) * t7525 + F::new(0.29896666666666666667e0) * t7531 + F::new(0.19931111111111111111e0) * t7527 - F::new(0.33218518518518518518e0) * t7535 - F::new(0.29896666666666666667e0) * t7550 - t7656 - t7657 - F::new(0.82156666666666666668e-1) * t7596 + F::new(0.49293999999999999999e0) * t7599 - F::new(0.59793333333333333333e0) * t7541 + F::new(0.17938e1) * t7547;
    t7662
}
