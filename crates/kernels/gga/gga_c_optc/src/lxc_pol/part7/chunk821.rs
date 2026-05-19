//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 821/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk821<F: Float>(t7592: F, t7523: F, t7525: F, t7527: F, t7531: F, t7535: F, t7541: F, t7547: F, t7550: F, t7576: F, t7580: F, t7583: F, t7596: F, t7599: F) -> F {
    let t7786 = F::cast_from(0.46308888888888888888e0_f64) * t7592;
    let t7787 = F::cast_from(0.16068111111111111111e1_f64) * t7523;
    let t7792 = F::cast_from(0.69463333333333333335e-1_f64) * t7576 - F::cast_from(0.46308888888888888889e-1_f64) * t7580 - F::new(0.104195e0) * t7583 - F::cast_from(0.68863333333333333332e0_f64) * t7525 + F::cast_from(0.51647499999999999999e0_f64) * t7531 + F::cast_from(0.34431666666666666666e0_f64) * t7527 - F::cast_from(0.57386111111111111112e0_f64) * t7535 - F::new(0.516475e0) * t7550 - t7786 - t7787 - F::new(0.104195e0) * t7596 + F::new(0.62517e0) * t7599 - F::new(0.103295e1) * t7541 + F::new(0.309885e1) * t7547;
    t7792
}
